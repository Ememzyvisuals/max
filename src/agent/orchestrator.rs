// agent/orchestrator.rs — ULTRAPLAN Multi-Agent Orchestration
// Created by Ememzyvisuals (Emmanuel Ariyo)

use anyhow::Result;
use colored::Colorize;
use tokio::task::JoinSet;

use crate::config::MaxConfig;
use crate::model::ModelRouter;
use crate::ui::spinner::Spinner;

#[derive(Debug, Clone)]
pub struct AgentTask {
    pub id: u8,
    pub role: String,
    pub objective: String,
    pub result: Option<String>,
}

/// ULTRAPLAN: Breaks a task into N parallel agent sub-tasks, executes
/// them concurrently, then synthesizes results into a master plan.
pub async fn ultraplan(task: String, num_agents: u8, config: &MaxConfig) -> Result<()> {
    println!();
    println!(
        "  {} {}",
        "ULTRAPLAN".bright_magenta().bold(),
        "— Multi-Agent Orchestration".bright_white()
    );
    println!("{}", "  ─────────────────────────────────────────────".bright_black());
    println!("  {} {}", "Task:".bright_cyan(), task.bright_white().bold());
    println!("  {} {}", "Agents:".bright_cyan(), num_agents.to_string().bright_yellow());
    println!();

    // Phase 1: Decompose task into sub-tasks
    let decompose_spinner = Spinner::new("Decomposing task into agent workstreams...");
    decompose_spinner.start();

    let router = ModelRouter::new(config);

    let json_example = r#"[{"id": 1, "role": "<role name>", "objective": "<specific objective>"}, ...]"#;
    let decompose_prompt = format!(
        "You are ULTRAPLAN, an expert task decomposition AI.\n\
         Break this task into exactly {} parallel sub-tasks for specialist agents.\n\
         Format your response as a JSON array:\n\
         {}\n\
         Respond ONLY with the JSON array, no other text.\n\
         Task: {}",
        num_agents, json_example, task
    );

    let decompose_result = router.complete(&decompose_prompt, config).await?;
    decompose_spinner.stop();

    // Parse agent tasks (with fallback)
    let agent_tasks = parse_agent_tasks(&decompose_result, num_agents, &task);

    // Show decomposition
    println!("{}", "  AGENT WORKSTREAMS".bright_cyan().bold());
    for agent in &agent_tasks {
        println!(
            "  {} {} — {}",
            format!("[Agent {}]", agent.id).bright_magenta(),
            agent.role.bright_white().bold(),
            agent.objective.bright_black()
        );
    }
    println!();

    // Phase 2: Execute agents in parallel
    println!("{}", "  EXECUTING AGENTS IN PARALLEL...".bright_yellow().bold());
    println!();

    let mut join_set: JoinSet<(u8, String, Result<String>)> = JoinSet::new();

    for agent in agent_tasks.clone() {
        let cfg = config.clone();
        let task_clone = task.clone();

        join_set.spawn(async move {
            let router = ModelRouter::new(&cfg);
            let prompt = format!(
                "You are Agent {} — {}.\n\
                 Parent task: {}\n\
                 Your specific objective: {}\n\
                 Provide a detailed, actionable analysis and recommendations.\n\
                 Be concise but thorough. Format with clear sections.",
                agent.id, agent.role, task_clone, agent.objective
            );

            let result = router.complete(&prompt, &cfg).await;
            (agent.id, agent.role.clone(), result)
        });
    }

    let mut results: Vec<(u8, String, String)> = Vec::new();

    while let Some(res) = join_set.join_next().await {
        match res {
            Ok((id, role, Ok(output))) => {
                println!(
                    "  {} {} completed",
                    format!("[Agent {}]", id).bright_green().bold(),
                    role.bright_white()
                );
                results.push((id, role, output));
            }
            Ok((id, role, Err(e))) => {
                eprintln!(
                    "  {} {} failed: {}",
                    format!("[Agent {}]", id).bright_red().bold(),
                    role.bright_white(),
                    e
                );
            }
            Err(e) => {
                eprintln!("  {} {}", "[JOIN ERROR]".bright_red(), e);
            }
        }
    }

    results.sort_by_key(|(id, _, _)| *id);

    // Phase 3: Synthesize results
    println!();
    let synth_spinner = Spinner::new("Synthesizing agent outputs into master plan...");
    synth_spinner.start();

    let mut synthesis_input = format!(
        "You are ULTRAPLAN Synthesizer.\n\
         Original task: {}\n\
         {} agents have completed their analysis. Synthesize into a comprehensive master plan.\n\n",
        task,
        results.len()
    );

    for (id, role, output) in &results {
        synthesis_input.push_str(&format!(
            "=== Agent {} ({}) ===\n{}\n\n",
            id, role, output
        ));
    }

    synthesis_input.push_str(
        "Create a clear, structured master plan with:\n\
         1. Executive Summary\n\
         2. Key Insights from agents\n\
         3. Prioritized Action Steps\n\
         4. Risk Considerations\n\
         5. Success Metrics",
    );

    let master_plan = router.complete(&synthesis_input, config).await?;
    synth_spinner.stop();

    // Print master plan
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════╗".bright_magenta()
    );
    println!(
        "{}",
        "  ║        ULTRAPLAN MASTER PLAN         ║".bright_magenta().bold()
    );
    println!(
        "{}",
        "  ╚══════════════════════════════════════╝".bright_magenta()
    );
    println!();

    for line in master_plan.lines() {
        if line.starts_with("##") || line.starts_with("**") {
            println!("  {}", line.bright_yellow().bold());
        } else if line.starts_with('-') || line.starts_with('*') {
            println!("  {}", line.bright_white());
        } else if line.trim().is_empty() {
            println!();
        } else {
            println!("  {}", line);
        }
    }

    println!();
    println!(
        "  {} ULTRAPLAN complete. {} agents coordinated.",
        "✓".bright_green().bold(),
        results.len().to_string().bright_cyan()
    );
    println!();

    Ok(())
}

fn parse_agent_tasks(json_str: &str, num_agents: u8, task: &str) -> Vec<AgentTask> {
    // Try to parse JSON
    let clean = json_str
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(clean) {
        if let Some(arr) = parsed.as_array() {
            let tasks: Vec<AgentTask> = arr
                .iter()
                .enumerate()
                .filter_map(|(i, item)| {
                    Some(AgentTask {
                        id: item["id"].as_u64().unwrap_or((i + 1) as u64) as u8,
                        role: item["role"].as_str().unwrap_or("Specialist").to_string(),
                        objective: item["objective"].as_str().unwrap_or(task).to_string(),
                        result: None,
                    })
                })
                .collect();

            if !tasks.is_empty() {
                return tasks;
            }
        }
    }

    // Fallback: generate generic agents
    let default_roles = vec![
        ("Research Analyst", "Research and gather information about"),
        ("Strategy Architect", "Design the optimal approach for"),
        ("Implementation Planner", "Create detailed execution steps for"),
        ("Risk Assessor", "Identify risks and mitigation strategies for"),
        ("Quality Reviewer", "Review and optimize the solution for"),
    ];

    (0..num_agents as usize)
        .map(|i| {
            let (role, prefix) = default_roles[i % default_roles.len()];
            AgentTask {
                id: (i + 1) as u8,
                role: role.to_string(),
                objective: format!("{} {}", prefix, task),
                result: None,
            }
        })
        .collect()
}
