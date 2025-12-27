use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[cfg(target_arch = "wasm32")]
use leptos::event_target_value;
#[cfg(target_arch = "wasm32")]
use leptos::*;

fn generate_secret_number() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}

#[cfg(not(target_arch = "wasm32"))]
fn run_cli() {
    println!("Guess the number!");

    let secret_number = generate_secret_number();

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let Ok(guess) = guess.trim().parse::<u32>() else {
            println!("Please type a number!");
            continue;
        };

        println!("You guessed: {guess}");

        match guess.cmp(&(secret_number as u32)) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn GuessingGame() -> impl IntoView {
    let (secret_number, set_secret_number) = create_signal(generate_secret_number());
    let (guess, set_guess) = create_signal(String::new());
    let (attempts, set_attempts) = create_signal(0usize);
    let (status, set_status) = create_signal(None::<Ordering>);
    let (feedback, set_feedback) =
        create_signal(String::from("Guess the number between 1 and 100."));

    let handle_guess = move |_| {
        let trimmed = guess().trim().to_string();

        let Ok(number) = trimmed.parse::<i32>() else {
            set_feedback(String::from("Please enter a valid number."));
            return;
        };

        let attempt_no = attempts() + 1;
        set_attempts(attempt_no);

        let comparison = number.cmp(&secret_number());
        set_status(Some(comparison));

        let feedback_message = match comparison {
            Ordering::Less => format!("Attempt #{attempt_no}: Too small!"),
            Ordering::Greater => format!("Attempt #{attempt_no}: Too big!"),
            Ordering::Equal => format!(
                "You guessed it in {attempt_no} attempt{}! Press reset to play again.",
                if attempt_no == 1 { "" } else { "s" }
            ),
        };

        set_feedback(feedback_message);
    };

    let reset_game = move |_| {
        set_secret_number(generate_secret_number());
        set_guess(String::new());
        set_attempts(0);
        set_status(None);
        set_feedback(String::from(
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
        <main class="page">
            <style>{styles}</style>
            <section class="card">
                <h1>Leptos Guessing Game</h1>
                <p class="description">Practice guessing the hidden number using instant feedback and a modern Leptos UI.</p>
                <p class="feedback">{move || feedback().clone()}</p>
                <div class="controls">
                    <input
                        type="number"
                        min="1"
                        max="100"
                        placeholder="Enter your guess"
                        prop:value=move || guess().clone()
                        on:input=move |ev| set_guess(event_target_value(&ev))
                        prop:disabled=move || status() == Some(Ordering::Equal)
                    />
                    <button
                        class="primary"
                        on:click=handle_guess
                        disabled=move || status() == Some(Ordering::Equal)
                    >
                        "Check guess"
                    </button>
                    <button class="secondary" on:click=reset_game>
                        "Reset"
                    </button>
                </div>
                <div class="meta">
                    <span class="badge">{"Attempts: "}{move || attempts()}</span>
                    <span class="badge">{"Status: "}{
                        move || match status() {
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

#[cfg(target_arch = "wasm32")]
fn main() {
    mount_to_body(|| view! { <GuessingGame/> })
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    run_cli();
}
