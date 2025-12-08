use std::{fs, path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result};
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, PathPromptOptions, Render, SharedString, WeakEntity, Window,
};
use log::error;
use notifications::status_toast::{StatusToast, ToastIcon};
use ui::{
    Button, ButtonStyle, Headline, HeadlineSize, IconName, Label, LabelSize, Vector, VectorName,
    prelude::*,
};
use ui_input::InputField;
use util::ResultExt;
use workspace::{self, OpenOptions, Workspace};

use crate::templates;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WizardStep {
    AppName,
    Framework,
    Template,
    Creating,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Framework {
    React,
    Nextjs,
    Vue,
    Angular,
    Svelte,
}

impl Framework {
    pub fn all() -> [Framework; 5] {
        [
            Framework::React,
            Framework::Nextjs,
            Framework::Vue,
            Framework::Angular,
            Framework::Svelte,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Framework::React => "React",
            Framework::Nextjs => "Next.js",
            Framework::Vue => "Vue",
            Framework::Angular => "Angular",
            Framework::Svelte => "Svelte",
        }
    }
}

impl Default for Framework {
    fn default() -> Self {
        Framework::React
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Template {
    HelloWorld,
    Counter,
    TicTacToe,
    Auction,
    Custom,
}

impl Template {
    pub fn all() -> [Template; 5] {
        [
            Template::HelloWorld,
            Template::Counter,
            Template::TicTacToe,
            Template::Auction,
            Template::Custom,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Template::HelloWorld => "Hello World",
            Template::Counter => "Counter",
            Template::TicTacToe => "Tic-Tac-Toe",
            Template::Auction => "Auction",
            Template::Custom => "Custom",
        }
    }
}

impl Default for Template {
    fn default() -> Self {
        Template::HelloWorld
    }
}

pub struct BitcoinAppWizard {
    step: WizardStep,
    framework: Framework,
    template: Template,
    generate_docs: bool,
    focus_handle: FocusHandle,
    app_name_input: Entity<InputField>,
    custom_description_input: Entity<InputField>,
    workspace: WeakEntity<Workspace>,
    _app_state: Arc<workspace::AppState>,
}

impl BitcoinAppWizard {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        app_state: Arc<workspace::AppState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let app_name_input = cx.new(|cx| {
            InputField::new(window, cx, "my-bitcoin-app")
                .label("App name")
                .label_min_width(px(96.))
        });
        let custom_description_input = cx.new(|cx| {
            InputField::new(window, cx, "")
                .label("Custom contract description (optional)")
                .label_min_width(px(96.))
        });

        Self {
            step: WizardStep::AppName,
            framework: Framework::default(),
            template: Template::default(),
            generate_docs: true,
            focus_handle,
            app_name_input,
            custom_description_input,
            workspace,
            _app_state: app_state,
        }
    }

    fn app_name(&self, cx: &Context<Self>) -> String {
        self.app_name_input
            .read(cx)
            .editor()
            .read(cx)
            .text(cx)
            .to_string()
    }

    fn custom_description(&self, cx: &Context<Self>) -> Option<String> {
        let text = self
            .custom_description_input
            .read(cx)
            .editor()
            .read(cx)
            .text(cx)
            .to_string();

        if text.trim().is_empty() {
            None
        } else {
            Some(text)
        }
    }

    fn select_framework(&mut self, framework: Framework, cx: &mut Context<Self>) {
        self.framework = framework;
        cx.notify();
    }

    fn select_template(&mut self, template: Template, cx: &mut Context<Self>) {
        self.template = template;
        cx.notify();
    }

    fn next_step(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.step = match self.step {
            WizardStep::AppName => WizardStep::Framework,
            WizardStep::Framework => WizardStep::Template,
            WizardStep::Template | WizardStep::Creating => {
                self.create_project(window, cx);
                WizardStep::Creating
            }
        };
        cx.notify();
    }

    fn previous_step(&mut self, cx: &mut Context<Self>) {
        self.step = match self.step {
            WizardStep::AppName => WizardStep::AppName,
            WizardStep::Framework => WizardStep::AppName,
            WizardStep::Template => WizardStep::Framework,
            WizardStep::Creating => WizardStep::Template,
        };
        cx.notify();
    }

    fn create_project(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.step = WizardStep::Creating;
        cx.notify();

        let app_name = self.app_name(cx);
        let framework = self.framework;
        let template = self.template;
        let custom_description = self.custom_description(cx);
        let old_workspace = self.workspace.clone();
        let generate_docs = self.generate_docs;
        let app_state = self._app_state.clone();

        let picker = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some(SharedString::from("Select folder for project")),
        });

        let contract_filename = crate::templates::contract_filename(template).to_string();

        let prompt_text = if let Some(ref desc) = custom_description {
            format!(
                "The project scaffold is already created with {} frontend and Yours Wallet integration. \
                DO NOT run any CLI commands like 'npm create' or 'npx create-vue'. \
                Instead, EDIT the existing files to implement: {}\n\n\
                CRITICAL RULES:\n\
                - NEVER import .scrypt.ts files directly into frontend components\n\
                - Contracts must be compiled: `npx scrypt-cli compile`\n\
                - Load compiled artifacts dynamically, not via direct import\n\
                - When adding npm packages, ALSO update package.json dependencies\n\
                - Use toRaw() when passing Vue reactive contract instances to SDK methods\n\
                - Use bindTxBuilder() for custom transaction building with ANYONECANPAY_SINGLE\n\n\
                Focus on:\n\
                1. Complete the smart contract logic in contracts/{}\n\
                2. Update the Game component to interact with the COMPILED contract artifact\n\
                3. Use YoursDirectSigner for settlement transactions\n\n\
                See AI_RULES.md for full guidelines on tx building, signer patterns, and common pitfalls.",
                framework.display_name(),
                desc,
                contract_filename
            )
        } else {
            format!(
                "The project scaffold is already created with {} frontend and Yours Wallet integration. \
                DO NOT run any CLI commands. \
                EDIT the existing files to complete the {} implementation.\n\n\
                CRITICAL RULES:\n\
                - NEVER import .scrypt.ts files directly into frontend components\n\
                - Contracts must be compiled: `npx scrypt-cli compile`\n\
                - Load compiled artifacts dynamically, not via direct import\n\
                - When adding npm packages, ALSO update package.json dependencies\n\
                - Use toRaw() when passing Vue reactive contract instances to SDK methods\n\
                - Use bindTxBuilder() for custom transaction building with ANYONECANPAY_SINGLE\n\n\
                Focus on:\n\
                1. Complete the smart contract logic in contracts/{}\n\
                2. Update the Game component to interact with the COMPILED contract artifact\n\n\
                See AI_RULES.md for full guidelines on tx building, signer patterns, and common pitfalls.",
                framework.display_name(),
                template.display_name(),
                contract_filename
            )
        };

        let task = window.spawn(cx, async move |cx| {
            let paths_result = picker.await;
            let Some(result) = paths_result.log_err() else {
                return;
            };

            let selected_base = match result {
                Ok(Some(paths)) if !paths.is_empty() => paths[0].clone(),
                Ok(_) => {
                    if let Some(workspace) = old_workspace.upgrade() {
                        let _ = workspace.update_in(cx, |workspace, _window, cx| {
                            let toast = StatusToast::new(
                                "No folder selected; project not created",
                                cx,
                                |this, _cx| {
                                    this.icon(ToastIcon::new(IconName::Warning))
                                        .dismiss_button(true)
                                },
                            );
                            workspace.toggle_status_toast(toast, cx);
                        });
                    }
                    return;
                }
                Err(err) => {
                    error!("Folder picker failed: {err:?}");
                    if let Some(workspace) = old_workspace.upgrade() {
                        let _ = workspace.update_in(cx, |workspace, _window, cx| {
                            let toast = StatusToast::new(
                                format!("Folder picker failed: {err}"),
                                cx,
                                |this, _cx| {
                                    this.icon(ToastIcon::new(IconName::Warning))
                                        .dismiss_button(true)
                                },
                            );
                            workspace.toggle_status_toast(toast, cx);
                        });
                    }
                    return;
                }
            };

            let scaffold_result = write_scaffold(
                &app_name,
                selected_base.clone(),
                framework,
                template,
                custom_description.as_deref(),
                generate_docs,
                &prompt_text,
            );

            let project_path = match scaffold_result {
                Ok(path) => path,
                Err(err) => {
                    error!("Failed to create project: {err:?}");
                    if let Some(workspace) = old_workspace.upgrade() {
                        let _ = workspace.update_in(cx, |workspace, _window, cx| {
                            let toast = StatusToast::new(
                                format!("Failed to create project: {err}"),
                                cx,
                                |this, _cx| {
                                    this.icon(ToastIcon::new(IconName::Warning))
                                        .dismiss_button(true)
                                },
                            );
                            workspace.toggle_status_toast(toast, cx);
                        });
                    }
                    return;
                }
            };

            // Use the module-level open_paths to get the new workspace window handle
            let open_task = cx.update(|_window, cx| {
                workspace::open_paths(
                    &[project_path.clone()],
                    app_state.clone(),
                    OpenOptions::default(),
                    cx,
                )
            });

            let open_task = match open_task {
                Ok(task) => task,
                Err(err) => {
                    error!("Failed to spawn open_paths task: {err:?}");
                    return;
                }
            };

            let (new_workspace_window, _) = match open_task.await {
                Ok(result) => result,
                Err(err) => {
                    error!("Failed to open workspace: {err:?}");
                    return;
                }
            };

            // Now use the NEW workspace window for opening the contract and agent panel
            let contract_path = project_path.join("contracts").join(&contract_filename);

            let open_contract_result = new_workspace_window.update(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    contract_path.clone(),
                    OpenOptions::default(),
                    window,
                    cx,
                )
            });

            if let Ok(contract_task) = open_contract_result {
                if let Err(err) = contract_task.await {
                    error!("Failed to open contract file: {err:?}");
                }
            }

            let _ = new_workspace_window.update(cx, |workspace, window, cx| {
                log::info!("Opening agent panel via toggle_panel_focus");
                let _ = workspace.toggle_panel_focus::<agent_ui::AgentPanel>(window, cx);

                if let Some(panel) = workspace.panel::<agent_ui::AgentPanel>(cx) {
                    panel.update(cx, |panel, cx| {
                        panel.set_prompt_text(&prompt_text, window, cx);
                    });
                }
            });

            let _ = new_workspace_window.update(cx, |workspace, _window, cx| {
                let toast = StatusToast::new(
                    format!("Created \"{}\" — ready to build with AI!", app_name),
                    cx,
                    |this, _cx| {
                        this.icon(ToastIcon::new(IconName::Sparkle))
                            .dismiss_button(true)
                    },
                );
                workspace.toggle_status_toast(toast, cx);
            });
        });
        task.detach();

        cx.emit(DismissEvent);
    }

    fn render_step_indicator(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let steps = [
            ("Name", WizardStep::AppName),
            ("Framework", WizardStep::Framework),
            ("Template", WizardStep::Template),
        ];

        h_flex()
            .gap_2()
            .children(steps.into_iter().map(|(label, step)| {
                let is_active = self.step == step;
                let style = if is_active {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                };
                Button::new(SharedString::from(label), label)
                    .style(style)
                    .disabled(true)
                    .full_width()
                    .when(is_active, |this| this.icon(IconName::Check))
                    .into_any_element()
            }))
    }

    fn render_app_name_step(&self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(
                Label::new("Give your project a name")
                    .size(LabelSize::Large)
                    .color(Color::Default),
            )
            .child(self.app_name_input.clone())
            .child(
                Label::new("A folder will be created in the current directory.")
                    .color(Color::Muted)
                    .size(LabelSize::Small),
            )
            .into_any_element()
    }

    fn render_framework_step(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                Headline::new("Choose your frontend framework").size(HeadlineSize::Small),
            )
            .child(
                v_flex().gap_1().children(Framework::all().iter().map(|framework| {
                    let is_selected = self.framework == *framework;
                    let framework_copy = *framework;
                    Button::new(
                        SharedString::from(format!("framework-{framework:?}")),
                        framework.display_name(),
                    )
                    .style(if is_selected {
                        ButtonStyle::Filled
                    } else {
                        ButtonStyle::Subtle
                    })
                    .when(is_selected, |this| this.icon(IconName::Check))
                    .full_width()
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.select_framework(framework_copy, cx);
                    }))
                    .into_any_element()
                })),
            )
            .into_any_element()
    }

    fn render_template_step(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(Headline::new("Pick a template").size(HeadlineSize::Small))
            .child(
                v_flex().gap_1().children(Template::all().iter().map(|template| {
                    let is_selected = self.template == *template;
                    let template_copy = *template;
                    Button::new(
                        SharedString::from(format!("template-{template:?}")),
                        template.display_name(),
                    )
                    .style(if is_selected {
                        ButtonStyle::Filled
                    } else {
                        ButtonStyle::Subtle
                    })
                    .when(is_selected, |this| this.icon(IconName::Check))
                    .full_width()
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.select_template(template_copy, cx);
                    }))
                    .into_any_element()
                })),
            )
            .child(
                div().child(self.custom_description_input.clone()).when(
                    self.template == Template::Custom,
                    |this| this,
                ),
            )
            .child(
                Button::new(
                    "toggle-docs",
                    if self.generate_docs {
                        "Include PRD & tasks"
                    } else {
                        "Skip PRD & tasks"
                    },
                )
                .style(if self.generate_docs {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                })
                .on_click(cx.listener(|this, _, _window, cx| {
                    this.generate_docs = !this.generate_docs;
                    cx.notify();
                })),
            )
            .into_any_element()
    }

    fn render_creating_step(&self) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(Headline::new("Creating project...").size(HeadlineSize::Small))
            .child(Label::new("Scaffolding your Bitcoin app").color(Color::Muted))
    }

    fn render_actions(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let can_go_back = self.step != WizardStep::AppName && self.step != WizardStep::Creating;
        let primary_label = match self.step {
            WizardStep::AppName | WizardStep::Framework => "Next",
            WizardStep::Template => "Create project",
            WizardStep::Creating => "Working...",
        };

        h_flex()
            .justify_end()
            .gap_2()
            .child(
                Button::new("back", "Back")
                    .style(ButtonStyle::Subtle)
                    .disabled(!can_go_back)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.previous_step(cx);
                    })),
            )
            .child(
                Button::new("next", primary_label)
                    .style(ButtonStyle::Filled)
                    .disabled(self.step == WizardStep::Creating)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.next_step(window, cx);
                    })),
            )
    }
}

impl EventEmitter<DismissEvent> for BitcoinAppWizard {}

impl Focusable for BitcoinAppWizard {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl workspace::ModalView for BitcoinAppWizard {}

impl Render for BitcoinAppWizard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content: AnyElement = match self.step {
            WizardStep::AppName => self.render_app_name_step(window, cx).into_any_element(),
            WizardStep::Framework => self.render_framework_step(cx).into_any_element(),
            WizardStep::Template => self.render_template_step(cx).into_any_element(),
            WizardStep::Creating => self.render_creating_step().into_any_element(),
        };

        v_flex()
            .id("bitcoin-app-wizard")
            .gap_3()
            .p_4()
            .w(px(640.))
            .bg(cx.theme().colors().surface_background)
            .elevation_3(cx)
            .track_focus(&self.focus_handle(cx))
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Vector::square(VectorName::ZedLogo, rems(2.)))
                    .child(
                        div().child(Headline::new("New Bitcoin App"))
                    .child(
                        Label::new("Scaffold a sCrypt dApp with Yours Wallet")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .italic(),
                    ),
                    ),
            )
            .child(self.render_step_indicator(cx))
            .child(content)
            .child(self.render_actions(window, cx))
    }
}

fn write_scaffold(
    app_name: &str,
    base_dir: PathBuf,
    framework: Framework,
    template: Template,
    custom_description: Option<&str>,
    generate_docs: bool,
    prompt_text: &str,
) -> Result<PathBuf> {
    let sanitized = app_name.trim();
    let app_folder = if sanitized.is_empty() {
        "bitcoin-app"
    } else {
        sanitized
    };

    let project_path = base_dir.join(app_folder);

    fs::create_dir_all(&project_path)
        .with_context(|| format!("create project folder {}", project_path.display()))?;
    fs::create_dir_all(project_path.join("contracts"))
        .context("create contracts folder")?;
    fs::create_dir_all(project_path.join("src/components"))
        .context("create src/components folder")?;
    fs::create_dir_all(project_path.join("src/lib"))
        .context("create src/lib folder")?;
    fs::create_dir_all(project_path.join("src/services"))
        .context("create src/services folder")?;

    let contract_filename = templates::contract_filename(template);
    let contract_source = templates::contract_source(template, custom_description);

    let mut files: Vec<(PathBuf, String)> = vec![
        (
            project_path.join(".env.example"),
            templates::ENV_EXAMPLE.to_string(),
        ),
        (
            project_path.join("README.md"),
            templates::readme(framework, template),
        ),
        (
            project_path.join("AI_RULES.md"),
            templates::AI_RULES.to_string(),
        ),
        (
            project_path.join("contracts").join(contract_filename),
            contract_source,
        ),
    ];

    if generate_docs {
        files.push((
            project_path.join("PRD.md"),
            format!("# Project Requirements\n\nPrompt:\n\n{}\n", prompt_text),
        ));
        files.push((
            project_path.join("tasks.md"),
            "- [ ] Implement sCrypt covenant contract\n- [ ] Wire wallet connect flow\n- [ ] Build game UI and state management\n- [ ] Add transaction signing and broadcasting\n- [ ] Style with Tailwind (customize as needed)\n- [ ] Test on testnet\n- [ ] Update README with deploy instructions\n".to_string(),
        ));
    }

    // Frontend files (includes package.json, vite config, components, etc.)
    for (path, contents) in templates::frontend_files(framework, contract_filename) {
        files.push((project_path.join(path), contents));
    }

    // Backend files (OAuth server)
    for (path, contents) in templates::backend_files() {
        files.push((project_path.join(path), contents));
    }

    for (path, contents) in files {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create directory {} for file {}", parent.display(), path.display()))?;
        }
        fs::write(&path, contents).with_context(|| format!("write file {}", path.display()))?;
    }

    Ok(project_path)
}

