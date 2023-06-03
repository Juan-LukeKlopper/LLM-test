use llm_chain::parameters;
use llm_chain::step::Step;
use llm_chain::traits::Executor as ExecutorTrait;
use llm_chain::{chains::sequential::Chain, prompt};
use llm_chain_openai::chatgpt::Executor;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor with the default settings
    let exec = Executor::new()?;

    // Create a chain of steps with two prompts
    let chain = Chain::new(vec![
        // First step: make a general guide to learning the selected programming language 
        Step::for_prompt_template(
            prompt!("You are a bot for guiding people who to becoming better a programmer", "Use science based research, along with incorporating meta-learning techniques to breakdown a list of what someone would need to do to become an industry expert in {{lang}} programming language")
        ),

        // Second step: Explain the information given using chunking
        Step::for_prompt_template(
            prompt!( "You are an IT industry expert who teaches people complex technological topics using chunking learning.", "Please explain {{text}} in detail and mention specific exercises and projects.")
        )
    ]);

    // Run the chain with the provided parameters
    let res = chain
        .run(
            // Create a Parameters object with key-value pairs for the placeholders
            parameters!("lang" => "Rust"),
            &exec,
        )
        .await
        .unwrap();

    // Print the result to the console
    println!("{}", res.to_immediate().await?.as_content());
    Ok(())
}
