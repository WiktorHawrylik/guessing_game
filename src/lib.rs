use leptos::event_target_value;
use leptos::*;
use rand::Rng;
use std::cmp::Ordering;

fn generate_secret_number() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}

#[component]
fn GuessingGame() -> impl IntoView {
    let (secret_number, set_secret_number) = create_signal(generate_secret_number());
    let (guess, set_guess) = create_signal(String::new());
    let (attempts, set_attempts) = create_signal(0usize);
    let (status, set_status) = create_signal(None::<Ordering>);
    let (feedback, set_feedback) =
        create_signal(String::from("Guess the number between 1 and 100."));

    let guess_handler = move || {
        let trimmed = guess.get().trim().to_string();

        let Ok(number) = trimmed.parse::<i32>() else {
            set_feedback.set(String::from("Please enter a valid number."));
            return;
        };

        if number < 1 || number > 100 {
            set_feedback.set(String::from("Please enter a number between 1 and 100."));
            return;
        }
        let attempt_no = attempts.get() + 1;
        set_attempts.set(attempt_no);

        let comparison = number.cmp(&secret_number.get());
        set_status.set(Some(comparison));

        let feedback_message = match comparison {
            Ordering::Less => format!("Attempt #{attempt_no}: Too small!"),
            Ordering::Greater => format!("Attempt #{attempt_no}: Too big!"),
            Ordering::Equal => format!(
                "You guessed it in {attempt_no} attempt{}! Press reset to play again.",
                if attempt_no == 1 { "" } else { "s" }
            ),
        };

        set_feedback.set(feedback_message);
    };

    let reset_game = move |_| {
        set_secret_number.set(generate_secret_number());
        set_guess.set(String::new());
        set_attempts.set(0);
        set_status.set(None);
        set_feedback.set(String::from(
            "New game started! Try a number between 1 and 100.",
        ));
    };

    let styles = r#"
        :root {
            font-family: 'Inter', system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            color: #0f172a;
            background-color: #0b1021;
        }

        body {
            margin: 0;
            min-height: 100vh;
        }

        @keyframes celebrate {
            0% {
                background: radial-gradient(circle at 20% 20%, #1e293b, #0b1021 45%),
                            radial-gradient(circle at 80% 0%, #0ea5e9, transparent 35%),
                            radial-gradient(circle at 10% 90%, #a855f7, transparent 45%);
            }
            25% {
                background: radial-gradient(circle at 30% 30%, #22d3ee, #0b1021 45%),
                            radial-gradient(circle at 70% 10%, #fbbf24, transparent 35%),
                            radial-gradient(circle at 20% 80%, #ec4899, transparent 45%);
            }
            50% {
                background: radial-gradient(circle at 50% 50%, #10b981, #0b1021 45%),
                            radial-gradient(circle at 60% 20%, #f87171, transparent 35%),
                            radial-gradient(circle at 30% 70%, #8b5cf6, transparent 45%);
            }
            75% {
                background: radial-gradient(circle at 40% 40%, #06b6d4, #0b1021 45%),
                            radial-gradient(circle at 75% 15%, #a78bfa, transparent 35%),
                            radial-gradient(circle at 15% 75%, #f97316, transparent 45%);
            }
            100% {
                background: radial-gradient(circle at 20% 20%, #1e293b, #0b1021 45%),
                            radial-gradient(circle at 80% 0%, #0ea5e9, transparent 35%),
                            radial-gradient(circle at 10% 90%, #a855f7, transparent 45%);
            }
        }

        main.page {
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            background: radial-gradient(circle at 20% 20%, #1e293b, #0b1021 45%),
                        radial-gradient(circle at 80% 0%, #0ea5e9, transparent 35%),
                        radial-gradient(circle at 10% 90%, #a855f7, transparent 45%);
            padding: 2rem;
            box-sizing: border-box;
            transition: background 0.3s ease;
        }

        main.page.won {
            animation: celebrate 2s ease-in-out;
        }

        section.card {
            background: rgba(255, 255, 255, 0.04);
            border: 1px solid rgba(255, 255, 255, 0.08);
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.35);
            backdrop-filter: blur(6px);
            border-radius: 18px;
            padding: 2rem;
            max-width: 520px;
            width: 100%;
            color: #e2e8f0;
        }

        h1 {
            margin: 0 0 0.5rem;
            font-size: 1.9rem;
            letter-spacing: -0.03em;
        }

        p.description {
            margin: 0 0 1.25rem;
            color: #cbd5e1;
            line-height: 1.6;
        }

        p.feedback {
            background: rgba(14, 165, 233, 0.15);
            border: 1px solid rgba(14, 165, 233, 0.35);
            color: #e0f2fe;
            padding: 0.85rem 1rem;
            border-radius: 12px;
            margin: 0 0 1.25rem;
            font-weight: 600;
        }

        p.feedback.won {
            background: rgba(34, 197, 94, 0.2);
            border: 1px solid rgba(34, 197, 94, 0.5);
            color: #86efac;
            animation: pulse 1s infinite;
        }

        @keyframes pulse {
            0%, 100% {
                opacity: 1;
            }
            50% {
                opacity: 0.7;
            }
        }

        .controls {
            display: flex;
            gap: 0.75rem;
            align-items: center;
            margin-bottom: 1rem;
            flex-wrap: wrap;
        }

        input[type="number"] {
            flex: 1 1 160px;
            padding: 0.75rem 0.9rem;
            border-radius: 12px;
            border: 1px solid rgba(255, 255, 255, 0.12);
            background: rgba(255, 255, 255, 0.04);
            color: #e2e8f0;
            font-size: 1rem;
        }

        input[type="number"]:focus {
            outline: 2px solid #0ea5e9;
            border-color: #0ea5e9;
        }

        button {
            border: none;
            border-radius: 12px;
            padding: 0.75rem 1.1rem;
            font-weight: 700;
            cursor: pointer;
            transition: transform 120ms ease, box-shadow 120ms ease, opacity 120ms ease;
        }

        button.primary {
            background: linear-gradient(120deg, #0ea5e9, #6366f1);
            color: #0b1021;
            box-shadow: 0 12px 30px rgba(14, 165, 233, 0.3);
        }

        button.secondary {
            background: rgba(255, 255, 255, 0.08);
            color: #e2e8f0;
            border: 1px solid rgba(255, 255, 255, 0.12);
        }

        button:disabled {
            opacity: 0.6;
            cursor: not-allowed;
            box-shadow: none;
        }

        button:not(:disabled):hover {
            transform: translateY(-1px);
        }

        .meta {
            display: flex;
            justify-content: space-between;
            color: #94a3b8;
            font-size: 0.95rem;
        }

        .badge {
            padding: 0.35rem 0.65rem;
            border-radius: 10px;
            background: rgba(255, 255, 255, 0.06);
            border: 1px solid rgba(255, 255, 255, 0.08);
        }
    "#;

    view! {
        <main class="page" class:won=move || status.get() == Some(Ordering::Equal)>
            <style>{styles}</style>
            <section class="card">
                <h1>Practice Luck</h1>
                <p class="description">I'm thinking of a number between 1 and 100. Can you guess it?</p>
                <p class="feedback" class:won=move || status.get() == Some(Ordering::Equal)>{move || feedback.get().clone()}</p>
                <div class="controls">
                    <input
                        type="number"
                        min="1"
                        max="100"
                        placeholder="Enter your guess"
                        prop:value=move || guess.get().clone()
                        on:input=move |ev| set_guess.set(event_target_value(&ev))
                        prop:disabled=move || status.get() == Some(Ordering::Equal)
                        on:keydown=move |ev| {
                            if ev.key() == "Enter" && status.get() != Some(Ordering::Equal) {
                                guess_handler();
                            }
                        }
                    />
                    <button
                        class="primary"
                        on:click=move |_| guess_handler()
                        disabled=move || status.get() == Some(Ordering::Equal)
                    >
                        "Check guess"
                    </button>
                    <button class="secondary" on:click=reset_game>
                        "Reset"
                    </button>
                </div>
                <div class="meta">
                    <span class="badge">{"Attempts: "}{move || attempts.get()}</span>
                    <span class="badge">{"Status: "}{
                        move || match status.get() {
                            Some(Ordering::Less) => "Too low".to_string(),
                            Some(Ordering::Greater) => "Too high".to_string(),
                            Some(Ordering::Equal) => "Winner!".to_string(),
                            None => "Waiting for guess".to_string(),
                        }
                    }</span>
                </div>
            </section>
        </main>
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    leptos::mount_to_body(|| view! { <GuessingGame/> });
}
