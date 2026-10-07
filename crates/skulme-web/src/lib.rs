pub mod catalog;
pub mod models;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use crate::catalog::{evaluate_solution, get_categories, get_problem_by_slug, get_problems, get_tutor_guidance};
use crate::models::{ConsoleTab, Problem, SupportedLanguage, TestResult};

fn load_solved_from_storage() -> Vec<String> {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(val)) = storage.get_item("skulme_solved_problems") {
                if let Ok(list) = serde_json::from_str::<Vec<String>>(&val) {
                    return list;
                }
            }
        }
    }
    vec![]
}

fn save_solved_to_storage(slug: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let mut list = load_solved_from_storage();
            if !list.contains(&slug.to_string()) {
                list.push(slug.to_string());
                if let Ok(serialized) = serde_json::to_string(&list) {
                    let _ = storage.set_item("skulme_solved_problems", &serialized);
                }
            }
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let all_problems = get_problems();
    let initial_slug = all_problems.first().map(|p| p.slug).unwrap_or("two-sum");

    let (active_slug, set_active_slug) = signal(initial_slug.to_string());
    let (selected_lang, set_selected_lang) = signal(SupportedLanguage::Rust);
    let (code, set_code) = signal(
        get_problem_by_slug(initial_slug)
            .map(|p| p.get_starter_code(SupportedLanguage::Rust).to_string())
            .unwrap_or_default(),
    );
    let (drawer_open, set_drawer_open) = signal(false);
    let (search_query, set_search_query) = signal(String::new());
    let (expanded_hint, set_expanded_hint) = signal(Option::<usize>::None);
    let (tutor_query, set_tutor_query) = signal(String::new());
    let (tutor_response, set_tutor_response) = signal(Option::<String>::None);
    let (active_tab, set_active_tab) = signal(ConsoleTab::Tests);
    let (test_results, set_test_results) = signal(Vec::<TestResult>::new());
    let (console_output, set_console_output) = signal(String::from("Ready to run tests."));
    let (is_running, set_is_running) = signal(false);
    let (all_passed, set_all_passed) = signal(false);
    let (solved_slugs, set_solved_slugs) = signal(load_solved_from_storage());

    let active_problem = Memo::new(move |_| {
        let slug = active_slug.get();
        get_problem_by_slug(&slug).unwrap_or_else(|| get_problems()[0].clone())
    });

    let select_problem = move |slug: &'static str| {
        set_active_slug.set(slug.to_string());
        if let Some(prob) = get_problem_by_slug(slug) {
            set_code.set(prob.get_starter_code(selected_lang.get()).to_string());
        }
        set_expanded_hint.set(None);
        set_tutor_response.set(None);
        set_test_results.set(vec![]);
        set_all_passed.set(false);
        set_console_output.set(format!("Switched to: {}", slug));
        set_drawer_open.set(false);
    };

    let on_language_change = move |lang: SupportedLanguage| {
        set_selected_lang.set(lang);
        let prob = active_problem.get();
        set_code.set(prob.get_starter_code(lang).to_string());
        set_console_output.set(format!("Language changed to {}", lang.display_name()));
    };

    let on_reset_code = move |_| {
        let prob = active_problem.get();
        set_code.set(prob.get_starter_code(selected_lang.get()).to_string());
        set_console_output.set(String::from("Code reset to starter template."));
    };

    let on_run_tests = move |_| {
        set_is_running.set(true);
        set_active_tab.set(ConsoleTab::Tests);
        let slug = active_slug.get();
        let lang = selected_lang.get();
        let current_code = code.get();

        let results = evaluate_solution(&slug, lang, &current_code);
        let passed = !results.is_empty() && results.iter().all(|r| r.passed);
        
        set_test_results.set(results.clone());
        set_all_passed.set(passed);
        set_is_running.set(false);

        if passed {
            save_solved_to_storage(&slug);
            let mut updated = solved_slugs.get();
            if !updated.contains(&slug) {
                updated.push(slug.clone());
                set_solved_slugs.set(updated);
            }
            set_console_output.set(format!("Success! All {} test cases passed.", results.len()));
        } else {
            let failed_count = results.iter().filter(|r| !r.passed).count();
            set_console_output.set(format!(
                "Tests finished: {} passed, {} failed. Check the Tests tab for details.",
                results.len() - failed_count,
                failed_count
            ));
        }
    };

    let on_ask_tutor = move |_| {
        let q = tutor_query.get();
        let slug = active_slug.get();
        if !q.trim().is_empty() {
            let reply = get_tutor_guidance(&slug, &q);
            set_tutor_response.set(Some(reply));
        } else {
            let reply = get_tutor_guidance(&slug, "");
            set_tutor_response.set(Some(reply));
        }
    };

    let on_next_problem = move |_| {
        let current_slug = active_slug.get();
        let problems = get_problems();
        let next = if let Some(pos) = problems.iter().position(|p| p.slug == current_slug) {
            let next_idx = (pos + 1) % problems.len();
            problems[next_idx].slug
        } else {
            problems[0].slug
        };
        select_problem(next);
    };

    view! {
        <div style="display: flex; flex-direction: column; height: 100vh; background: #0b0f19; color: #f1f5f9; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;">
            // Top Navigation Bar
            <nav style="display: flex; align-items: center; justify-content: space-between; padding: 10px 20px; background: #111827; border-bottom: 1px solid #1f2937; height: 52px; box-sizing: border-box;">
                <div style="display: flex; align-items: center; gap: 16px;">
                    <button
                        style="background: #1f2937; color: #e2e8f0; border: 1px solid #374151; padding: 6px 14px; border-radius: 6px; cursor: pointer; font-size: 0.85rem; font-weight: 600; display: flex; align-items: center; gap: 6px;"
                        on:click=move |_| set_drawer_open.update(|open| *open = !*open)
                    >
                        <span>"☰"</span>
                        <span>"Problems"</span>
                    </button>
                    <div style="display: flex; align-items: center; gap: 8px;">
                        <span style="font-weight: 800; font-size: 1.15rem; color: #38bdf8; letter-spacing: -0.5px;">"Skul.me"</span>
                        <span style="color: #4b5563;">"|"</span>
                        <span style="font-weight: 600; font-size: 0.95rem; color: #f3f4f6;">
                            {move || active_problem.get().title}
                        </span>
                        <span style="background: #1e293b; color: #94a3b8; font-size: 0.75rem; padding: 2px 8px; border-radius: 9999px;">
                            {move || active_problem.get().category}
                        </span>
                    </div>
                </div>

                <div style="display: flex; align-items: center; gap: 14px;">
                    <div style="background: #1e293b; border: 1px solid #334155; padding: 4px 12px; border-radius: 9999px; font-size: 0.8rem; font-weight: 600; color: #4ade80;">
                        {move || format!("✓ Solved: {} / 394", solved_slugs.get().len())}
                    </div>
                    <button
                        style="background: #0284c7; color: white; border: none; padding: 6px 14px; border-radius: 6px; cursor: pointer; font-size: 0.85rem; font-weight: 600; display: flex; align-items: center; gap: 6px;"
                        on:click=on_next_problem
                    >
                        <span>"Next Problem"</span>
                        <span>"▶"</span>
                    </button>
                </div>
            </nav>

            // Main Layout Container
            <div style="display: flex; flex: 1; overflow: hidden; position: relative;">
                // Collapsible Problems Drawer
                {move || if drawer_open.get() {
                    let search_val = search_query.get().to_lowercase();
                    let problems_list = get_problems();
                    let filtered: Vec<Problem> = problems_list
                        .into_iter()
                        .filter(|p| p.title.to_lowercase().contains(&search_val) || p.category.to_lowercase().contains(&search_val))
                        .collect();
                    let categories = get_categories();

                    view! {
                        <div style="position: absolute; top: 0; left: 0; width: 340px; height: 100%; background: #0f172a; border-right: 1px solid #1e293b; z-index: 50; display: flex; flex-direction: column; box-shadow: 4px 0 16px rgba(0,0,0,0.5);">
                            <div style="padding: 14px; border-bottom: 1px solid #1e293b; display: flex; justify-content: space-between; align-items: center;">
                                <span style="font-weight: 700; font-size: 0.95rem; color: #e2e8f0;">"Problem Directory"</span>
                                <button
                                    style="background: transparent; border: none; color: #94a3b8; font-size: 1.1rem; cursor: pointer;"
                                    on:click=move |_| set_drawer_open.set(false)
                                >
                                    "✕"
                                </button>
                            </div>

                            <div style="padding: 12px; border-bottom: 1px solid #1e293b;">
                                <input
                                    type="text"
                                    placeholder="Search problems or categories..."
                                    style="width: 100%; background: #1e293b; border: 1px solid #334155; padding: 8px 12px; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; box-sizing: border-box;"
                                    on:input=move |ev| set_search_query.set(event_target_value(&ev))
                                    prop:value=search_query
                                />
                            </div>

                            <div style="flex: 1; overflow-y: auto; padding: 10px;">
                                <div style="font-size: 0.75rem; text-transform: uppercase; color: #64748b; font-weight: 700; padding: 6px 8px; margin-bottom: 4px;">
                                    "Topics & Problems"
                                </div>
                                {filtered.into_iter().map(|prob| {
                                    let slug = prob.slug;
                                    let is_solved = solved_slugs.get().contains(&slug.to_string());
                                    let is_active = active_slug.get() == slug;
                                    let diff_color = match prob.difficulty {
                                        "Easy" => "#4ade80",
                                        "Medium" => "#fbbf24",
                                        _ => "#f87171",
                                    };

                                    view! {
                                        <div
                                            style=format!(
                                                "display: flex; align-items: center; justify-content: space-between; padding: 10px 12px; margin-bottom: 4px; border-radius: 6px; cursor: pointer; background: {}; border-left: 3px solid {};",
                                                if is_active { "#1e293b" } else { "transparent" },
                                                if is_active { "#38bdf8" } else { "transparent" }
                                            )
                                            on:click=move |_| select_problem(slug)
                                        >
                                            <div style="display: flex; align-items: center; gap: 8px;">
                                                <span style=format!("font-size: 0.85rem; font-weight: bold; color: {};", if is_solved { "#4ade80" } else { "#64748b" })>
                                                    {if is_solved { "✓" } else { "○" }}
                                                </span>
                                                <span style="font-size: 0.9rem; font-weight: 500; color: #f1f5f9;">
                                                    {prob.title}
                                                </span>
                                            </div>
                                            <span style=format!("font-size: 0.75rem; font-weight: 600; color: {};", diff_color)>
                                                {prob.difficulty}
                                            </span>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}

                                <div style="margin-top: 16px; border-top: 1px solid #1e293b; padding-top: 12px;">
                                    <div style="font-size: 0.75rem; text-transform: uppercase; color: #64748b; font-weight: 700; padding: 6px 8px;">
                                        "Browse by Category"
                                    </div>
                                    {categories.into_iter().map(|(cat, count)| {
                                        view! {
                                            <div style="display: flex; justify-content: space-between; padding: 6px 8px; font-size: 0.8rem; color: #94a3b8;">
                                                <span>{cat}</span>
                                                <span style="background: #1e293b; padding: 1px 6px; border-radius: 4px; font-size: 0.75rem;">{count}</span>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

                // Left Pane: Problem Description & Socratic Hints
                <div style="flex: 1; min-width: 380px; max-width: 50%; border-right: 1px solid #1f2937; display: flex; flex-direction: column; background: #0f172a; overflow-y: auto;">
                    <div style="padding: 24px;">
                        // Problem Header Badges
                        <div style="display: flex; align-items: center; gap: 10px; margin-bottom: 12px;">
                            <span style=format!(
                                "font-size: 0.8rem; font-weight: 700; padding: 2px 10px; border-radius: 9999px; background: {}; color: {};",
                                match active_problem.get().difficulty {
                                    "Easy" => "rgba(74, 222, 128, 0.15)",
                                    "Medium" => "rgba(251, 191, 36, 0.15)",
                                    _ => "rgba(248, 113, 113, 0.15)",
                                },
                                match active_problem.get().difficulty {
                                    "Easy" => "#4ade80",
                                    "Medium" => "#fbbf24",
                                    _ => "#f87171",
                                }
                            )>
                                {move || active_problem.get().difficulty}
                            </span>
                            <span style="font-size: 0.8rem; color: #94a3b8; background: #1e293b; padding: 2px 10px; border-radius: 9999px;">
                                {move || format!("Time: {}", active_problem.get().time_complexity)}
                            </span>
                            <span style="font-size: 0.8rem; color: #94a3b8; background: #1e293b; padding: 2px 10px; border-radius: 9999px;">
                                {move || format!("Space: {}", active_problem.get().space_complexity)}
                            </span>
                        </div>

                        // Problem Title & Description
                        <h1 style="font-size: 1.45rem; font-weight: 700; color: #f8fafc; margin: 0 0 16px 0;">
                            {move || active_problem.get().title}
                        </h1>
                        <p style="font-size: 0.95rem; line-height: 1.6; color: #cbd5e1; margin: 0 0 24px 0; white-space: pre-line;">
                            {move || active_problem.get().description}
                        </p>

                        // Examples Section
                        <div style="margin-bottom: 24px;">
                            <h3 style="font-size: 0.95rem; font-weight: 700; color: #e2e8f0; margin: 0 0 12px 0;">"Examples"</h3>
                            {move || {
                                let examples = active_problem.get().examples;
                                examples.into_iter().enumerate().map(|(idx, ex)| {
                                    view! {
                                        <div style="background: #1e293b; border: 1px solid #334155; border-radius: 6px; padding: 12px; margin-bottom: 10px; font-size: 0.85rem;">
                                            <div style="font-weight: 600; color: #94a3b8; margin-bottom: 6px;">
                                                {format!("Example {}", idx + 1)}
                                            </div>
                                            <div style="margin-bottom: 4px;">
                                                <span style="color: #64748b; font-weight: 600;">"Input: "</span>
                                                <code style="color: #38bdf8; font-family: monospace;">{ex.input}</code>
                                            </div>
                                            <div style="margin-bottom: 4px;">
                                                <span style="color: #64748b; font-weight: 600;">"Output: "</span>
                                                <code style="color: #4ade80; font-family: monospace;">{ex.output}</code>
                                            </div>
                                            {if let Some(exp) = ex.explanation {
                                                view! {
                                                    <div>
                                                        <span style="color: #64748b; font-weight: 600;">"Explanation: "</span>
                                                        <span style="color: #cbd5e1;">{exp}</span>
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <div></div> }.into_any()
                                            }}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()
                            }}
                        </div>

                        // Constraints Section
                        <div style="margin-bottom: 28px;">
                            <h3 style="font-size: 0.95rem; font-weight: 700; color: #e2e8f0; margin: 0 0 8px 0;">"Constraints"</h3>
                            <ul style="margin: 0; padding-left: 20px; font-size: 0.85rem; color: #94a3b8; line-height: 1.6;">
                                {move || {
                                    let constraints = active_problem.get().constraints;
                                    constraints.into_iter().map(|c| {
                                        view! { <li><code style="color: #cbd5e1; font-family: monospace;">{c}</code></li> }
                                    }).collect::<Vec<_>>()
                                }}
                            </ul>
                        </div>

                        // Expandable Hints Section
                        <div style="margin-bottom: 28px;">
                            <h3 style="font-size: 0.95rem; font-weight: 700; color: #e2e8f0; margin: 0 0 10px 0;">"Hints"</h3>
                            {move || {
                                let hints = active_problem.get().hints;
                                hints.into_iter().enumerate().map(|(idx, hint)| {
                                    let is_open = expanded_hint.get() == Some(idx);
                                    view! {
                                        <div style="background: #111827; border: 1px solid #1f2937; border-radius: 6px; margin-bottom: 8px; overflow: hidden;">
                                            <button
                                                style="width: 100%; text-align: left; background: transparent; border: none; padding: 10px 14px; color: #38bdf8; font-size: 0.85rem; font-weight: 600; cursor: pointer; display: flex; justify-content: space-between; align-items: center;"
                                                on:click=move |_| {
                                                    set_expanded_hint.update(|cur| {
                                                        *cur = if *cur == Some(idx) { None } else { Some(idx) };
                                                    });
                                                }
                                            >
                                                <span>{hint.title}</span>
                                                <span>{if is_open { "▲" } else { "▼" }}</span>
                                            </button>
                                            {if is_open {
                                                view! {
                                                    <div style="padding: 10px 14px; background: #0f172a; border-top: 1px solid #1f2937; font-size: 0.85rem; color: #cbd5e1; line-height: 1.5;">
                                                        {hint.content}
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <div></div> }.into_any()
                                            }}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()
                            }}
                        </div>

                        // Socratic Ask Tutor Section
                        <div style="background: #111827; border: 1px solid #1f2937; border-radius: 8px; padding: 16px;">
                            <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 10px;">
                                <span style="font-size: 1.1rem;">"💡"</span>
                                <span style="font-weight: 700; font-size: 0.9rem; color: #e2e8f0;">"Ask Tutor"</span>
                                <span style="font-size: 0.75rem; color: #64748b;">"(Guided inquiry)"</span>
                            </div>
                            <div style="display: flex; gap: 8px;">
                                <input
                                    type="text"
                                    placeholder="e.g. How can I optimize time complexity?"
                                    style="flex: 1; background: #1e293b; border: 1px solid #334155; padding: 8px 12px; border-radius: 6px; color: #f8fafc; font-size: 0.85rem;"
                                    on:input=move |ev| set_tutor_query.set(event_target_value(&ev))
                                    prop:value=tutor_query
                                />
                                <button
                                    style="background: #3b82f6; color: white; border: none; padding: 8px 16px; border-radius: 6px; cursor: pointer; font-size: 0.85rem; font-weight: 600;"
                                    on:click=on_ask_tutor
                                >
                                    "Ask"
                                </button>
                            </div>
                            {move || if let Some(reply) = tutor_response.get() {
                                view! {
                                    <div style="margin-top: 12px; background: #0f172a; border-left: 3px solid #a855f7; padding: 10px 12px; border-radius: 0 6px 6px 0; font-size: 0.85rem; color: #e2e8f0; line-height: 1.5;">
                                        <div style="font-weight: 600; color: #c084fc; margin-bottom: 4px;">"Tutor:"</div>
                                        <div>{reply}</div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}
                        </div>
                    </div>
                </div>

                // Right Pane: Code Editor & Console Workspace
                <div style="flex: 1; display: flex; flex-direction: column; background: #0d1117; overflow: hidden;">
                    // Editor Header & Language Selector
                    <div style="display: flex; align-items: center; justify-content: space-between; padding: 8px 16px; background: #161b22; border-bottom: 1px solid #30363d; height: 44px; box-sizing: border-box;">
                        // Language Selector Buttons
                        <div style="display: flex; gap: 6px;">
                            {[SupportedLanguage::Rust, SupportedLanguage::Python, SupportedLanguage::Java, SupportedLanguage::TypeScript].into_iter().map(|lang| {
                                let is_sel = selected_lang.get() == lang;
                                view! {
                                    <button
                                        style=format!(
                                            "padding: 4px 10px; border-radius: 4px; font-size: 0.8rem; font-weight: 600; cursor: pointer; border: none; background: {}; color: {};",
                                            if is_sel { "#238636" } else { "#21262d" },
                                            if is_sel { "#ffffff" } else { "#8b949e" }
                                        )
                                        on:click=move |_| on_language_change(lang)
                                    >
                                        {lang.display_name()}
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>

                        // Action Buttons: Reset & Run
                        <div style="display: flex; gap: 8px; align-items: center;">
                            <button
                                style="background: #21262d; color: #c9d1d9; border: 1px solid #30363d; padding: 5px 12px; border-radius: 6px; cursor: pointer; font-size: 0.8rem; font-weight: 500;"
                                on:click=on_reset_code
                            >
                                "Reset"
                            </button>
                            <button
                                style="background: #238636; color: white; border: none; padding: 6px 18px; border-radius: 6px; cursor: pointer; font-size: 0.85rem; font-weight: 700; display: flex; align-items: center; gap: 6px;"
                                on:click=on_run_tests
                                disabled=is_running
                            >
                                <span>{if is_running.get() { "Running..." } else { "Run Tests ▶" }}</span>
                            </button>
                        </div>
                    </div>

                    // Code Area
                    <div style="flex: 1; position: relative; background: #0d1117;">
                        <textarea
                            style="width: 100%; height: 100%; box-sizing: border-box; background: #0d1117; color: #c9d1d9; font-family: 'Fira Code', 'Consolas', 'Courier New', monospace; font-size: 0.95rem; line-height: 1.5; padding: 16px; border: none; outline: none; resize: none; tab-size: 4;"
                            on:input=move |ev| set_code.set(event_target_value(&ev))
                            prop:value=code
                        />
                    </div>

                    // Bottom Console & Test Results Pane
                    <div style="height: 250px; background: #161b22; border-top: 1px solid #30363d; display: flex; flex-direction: column;">
                        // Tabs Header
                        <div style="display: flex; align-items: center; justify-content: space-between; padding: 0 16px; background: #0d1117; border-bottom: 1px solid #30363d; height: 36px;">
                            <div style="display: flex; gap: 12px;">
                                <button
                                    style=format!(
                                        "background: transparent; border: none; cursor: pointer; font-size: 0.8rem; font-weight: 600; padding: 8px 4px; border-bottom: 2px solid {}; color: {};",
                                        if active_tab.get() == ConsoleTab::Tests { "#58a6ff" } else { "transparent" },
                                        if active_tab.get() == ConsoleTab::Tests { "#f0f6fc" } else { "#8b949e" }
                                    )
                                    on:click=move |_| set_active_tab.set(ConsoleTab::Tests)
                                >
                                    "Tests"
                                </button>
                                <button
                                    style=format!(
                                        "background: transparent; border: none; cursor: pointer; font-size: 0.8rem; font-weight: 600; padding: 8px 4px; border-bottom: 2px solid {}; color: {};",
                                        if active_tab.get() == ConsoleTab::Console { "#58a6ff" } else { "transparent" },
                                        if active_tab.get() == ConsoleTab::Console { "#f0f6fc" } else { "#8b949e" }
                                    )
                                    on:click=move |_| set_active_tab.set(ConsoleTab::Console)
                                >
                                    "Console"
                                </button>
                            </div>

                            {move || if all_passed.get() {
                                view! {
                                    <div style="display: flex; align-items: center; gap: 8px;">
                                        <span style="font-size: 0.8rem; font-weight: 700; color: #3fb950;">
                                            "🎉 Passed All Tests!"
                                        </span>
                                        <button
                                            style="background: #1f6feb; color: white; border: none; padding: 3px 10px; border-radius: 4px; font-size: 0.75rem; font-weight: 600; cursor: pointer;"
                                            on:click=on_next_problem
                                        >
                                            "Next Problem ▶"
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}
                        </div>

                        // Tab Content
                        <div style="flex: 1; padding: 12px 16px; overflow-y: auto; font-family: monospace; font-size: 0.85rem;">
                            {move || match active_tab.get() {
                                ConsoleTab::Tests => {
                                    let results = test_results.get();
                                    if results.is_empty() {
                                        view! {
                                            <div style="color: #8b949e; padding-top: 12px; font-style: italic;">
                                                "Click 'Run Tests ▶' above to evaluate your code against the test harness."
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div>
                                                {results.into_iter().map(|res| {
                                                    let is_pass = res.passed;
                                                    view! {
                                                        <div style=format!(
                                                            "background: {}; border: 1px solid {}; border-radius: 6px; padding: 10px; margin-bottom: 8px;",
                                                            if is_pass { "rgba(46, 160, 67, 0.1)" } else { "rgba(248, 81, 73, 0.1)" },
                                                            if is_pass { "rgba(46, 160, 67, 0.3)" } else { "rgba(248, 81, 73, 0.3)" }
                                                        )>
                                                            <div style="display: flex; justify-content: space-between; margin-bottom: 4px;">
                                                                <span style=format!("font-weight: 700; color: {};", if is_pass { "#3fb950" } else { "#f85149" })>
                                                                    {if is_pass { format!("✓ {} ({}ms)", res.name, res.duration_ms) } else { format!("✗ {}", res.name) }}
                                                                </span>
                                                            </div>
                                                            <div style="color: #8b949e; font-size: 0.8rem; margin-bottom: 2px;">
                                                                <span>"Input: "</span><span style="color: #c9d1d9;">{res.input}</span>
                                                            </div>
                                                            <div style="color: #8b949e; font-size: 0.8rem; margin-bottom: 2px;">
                                                                <span>"Expected: "</span><span style="color: #3fb950;">{res.expected}</span>
                                                            </div>
                                                            <div style="color: #8b949e; font-size: 0.8rem;">
                                                                <span>"Actual: "</span>
                                                                <span style=format!("color: {};", if is_pass { "#3fb950" } else { "#f85149" })>
                                                                    {res.actual}
                                                                </span>
                                                            </div>
                                                            {if let Some(err) = res.error {
                                                                view! {
                                                                    <div style="color: #f85149; margin-top: 4px; font-size: 0.75rem;">
                                                                        {err}
                                                                    </div>
                                                                }.into_any()
                                                            } else {
                                                                view! { <div></div> }.into_any()
                                                            }}
                                                        </div>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        }.into_any()
                                    }
                                },
                                ConsoleTab::Console => {
                                    let output_text = console_output.get();
                                    view! {
                                        <div style="color: #c9d1d9; white-space: pre-wrap; line-height: 1.5;">
                                            {output_text}
                                        </div>
                                    }.into_any()
                                },
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
