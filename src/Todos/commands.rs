use super::structs::TodoState;

#[derive(clap::Args)]
pub struct TodoArgs {
    #[clap(subcommand)]
    pub command: Option<TodoCommand>,

    #[clap(short, long)]
    pub todo: bool,

    #[clap(short, long)]
    pub group: bool,
}

#[derive(clap::Subcommand)]
pub enum TodoCommand {
    Add {
        #[clap(short, long)]
        name: String,
        /// documentation for the todo
        #[clap(short, long)]
        documentation: Option<String>,
        /// group id for the todo
        #[clap(short, long)]
        group_id: Option<i32>,
    },
    List {
        #[clap(short, long)]
        all: bool,
        #[clap(short, long)]
        group_id: Option<i32>,
        todo_state: Option<TodoState>,
    },
    /// only works for todos
    SetState {
        #[clap(short, long)]
        id: i32,
        #[clap(short, long)]
        state: TodoState,
    },
    Remove {
        id: i32,
    },
}
