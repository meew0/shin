#![allow(clippy::upper_case_acronyms)]

mod prelude {
    pub use crate::adv::AdvState;
    pub use crate::layer::Layer;
    pub use crate::update::UpdateContext;
    pub use crate::vm::command::CommandStartResult;
    pub use crate::vm::VmState;
    pub use shin_core::vm::command;
    pub use shin_core::vm::command::layer::VLayerIdRepr;
    pub use shin_core::vm::command::CommandResult;
    pub use tracing::warn;
}

mod autosave;
mod bgmplay;
mod bgmstop;
mod layerctrl;
mod layerinit;
mod layerload;
mod layerunload;
mod msgclose;
mod msginit;
mod msgset;
mod pageback;
mod saveinfo;
mod seplay;
mod sestopall;
mod sget;
mod sset;
mod wait;
mod wipe;

use msgset::MSGSET;
use wait::WAIT;

use enum_dispatch::enum_dispatch;

use shin_core::vm::command::{CommandResult, RuntimeCommand};

use crate::adv::AdvState;
use crate::update::UpdateContext;
use crate::vm::VmState;

#[enum_dispatch]
pub trait UpdatableCommand {
    // TODO: provide mutable access to Adv Scene state
    fn update(
        &mut self,
        context: &UpdateContext,
        vm_state: &VmState,
        adv_state: &mut AdvState,
    ) -> Option<CommandResult>;
}

// all commands that yield to the game loop should have:
// - a type implementing UpdatableCommand
// - a enum variant for that type here
#[enum_dispatch(UpdatableCommand)]
pub enum ExecutingCommand {
    WAIT,
    MSGSET,
}

impl StartableCommand for RuntimeCommand {
    fn apply_state(&self, state: &mut VmState) {
        match self {
            // RuntimeCommand::EXIT(v) => v.apply_state(state),
            RuntimeCommand::SGET(v) => v.apply_state(state),
            RuntimeCommand::SSET(v) => v.apply_state(state),
            RuntimeCommand::WAIT(v) => v.apply_state(state),
            RuntimeCommand::MSGINIT(v) => v.apply_state(state),
            RuntimeCommand::MSGSET(v) => v.apply_state(state),
            // RuntimeCommand::MSGWAIT(v) => v.apply_state(state),
            // RuntimeCommand::MSGSIGNAL(v) => v.apply_state(state),
            // RuntimeCommand::MSGSYNC(v) => v.apply_state(state),
            RuntimeCommand::MSGCLOSE(v) => v.apply_state(state),
            // RuntimeCommand::SELECT(v) => v.apply_state(state),
            RuntimeCommand::WIPE(v) => v.apply_state(state),
            // RuntimeCommand::WIPEWAIT(v) => v.apply_state(state),
            RuntimeCommand::BGMPLAY(v) => v.apply_state(state),
            RuntimeCommand::BGMSTOP(v) => v.apply_state(state),
            // RuntimeCommand::BGMVOL(v) => v.apply_state(state),
            // RuntimeCommand::BGMWAIT(v) => v.apply_state(state),
            // RuntimeCommand::BGMSYNC(v) => {}
            // RuntimeCommand::SEPLAY(v) => {}
            // RuntimeCommand::SESTOP(v) => {}
            // RuntimeCommand::SESTOPALL(v) => {}
            // RuntimeCommand::SEVOL(v) => {}
            // RuntimeCommand::SEPAN(v) => {}
            // RuntimeCommand::SEWAIT(v) => {}
            // RuntimeCommand::SEONCE(v) => {}
            // RuntimeCommand::VOICEPLAY(v) => {}
            // RuntimeCommand::VOICESTOP(v) => {}
            // RuntimeCommand::VOICEWAIT(v) => {}
            // RuntimeCommand::SYSSE(v) => {}
            // RuntimeCommand::SAVEINFO(v) => {}
            // RuntimeCommand::AUTOSAVE(v) => {}
            // RuntimeCommand::EVBEGIN(v) => {}
            // RuntimeCommand::EVEND(v) => {}
            // RuntimeCommand::RESUMESET(v) => {}
            // RuntimeCommand::RESUME(v) => {}
            // RuntimeCommand::SYSCALL(v) => {}
            // RuntimeCommand::TROPHY(v) => {}
            // RuntimeCommand::UNLOCK(v) => {}
            RuntimeCommand::LAYERINIT(v) => v.apply_state(state),
            RuntimeCommand::LAYERLOAD(v) => v.apply_state(state),
            RuntimeCommand::LAYERUNLOAD(v) => v.apply_state(state),
            RuntimeCommand::LAYERCTRL(v) => v.apply_state(state),
            // RuntimeCommand::LAYERWAIT(v) => {}
            // RuntimeCommand::LAYERSWAP(v) => {}
            // RuntimeCommand::LAYERSELECT(v) => {}
            // RuntimeCommand::MOVIEWAIT(v) => {}
            // RuntimeCommand::TRANSSET(v) => {}
            // RuntimeCommand::TRANSWAIT(v) => {}
            // RuntimeCommand::PAGEBACK(v) => {}
            // RuntimeCommand::PLANESELECT(v) => {}
            // RuntimeCommand::PLANECLEAR(v) => {}
            // RuntimeCommand::MASKLOAD(v) => {}
            // RuntimeCommand::MASKUNLOAD(v) => {}
            // RuntimeCommand::CHARS(v) => {}
            // RuntimeCommand::TIPSGET(v) => {}
            // RuntimeCommand::QUIZ(v) => {}
            // RuntimeCommand::SHOWCHARS(v) => {}
            // RuntimeCommand::NOTIFYSET(v) => {}
            // RuntimeCommand::DEBUGOUT(v) => {}
            _ => todo!(),
        }
    }

    fn start(
        self,
        context: &UpdateContext,
        vm_state: &VmState,
        adv_state: &mut AdvState,
    ) -> CommandStartResult {
        match self {
            // RuntimeCommand::EXIT(v) => v.start(vm),
            RuntimeCommand::SGET(v) => v.start(context, vm_state, adv_state),
            RuntimeCommand::SSET(v) => v.start(context, vm_state, adv_state),
            RuntimeCommand::WAIT(v) => v.start(context, vm_state, adv_state),
            RuntimeCommand::MSGINIT(v) => v.start(context, vm_state, adv_state),
            RuntimeCommand::MSGSET(v) => v.start(context, vm_state, adv_state),
            // ---
            // RuntimeCommand::BGMSYNC(v) => {}
            // RuntimeCommand::SEPLAY(v) => {}
            // RuntimeCommand::SESTOP(v) => {}
            // RuntimeCommand::SESTOPALL(v) => {}
            // RuntimeCommand::SEVOL(v) => {}
            // RuntimeCommand::SEPAN(v) => {}
            // RuntimeCommand::SEWAIT(v) => {}
            // RuntimeCommand::SEONCE(v) => {}
            // RuntimeCommand::VOICEPLAY(v) => {}
            // RuntimeCommand::VOICESTOP(v) => {}
            // RuntimeCommand::VOICEWAIT(v) => {}
            // RuntimeCommand::SYSSE(v) => {}
            // RuntimeCommand::SAVEINFO(v) => {}
            // RuntimeCommand::AUTOSAVE(v) => {}
            // RuntimeCommand::EVBEGIN(v) => {}
            // RuntimeCommand::EVEND(v) => {}
            // RuntimeCommand::RESUMESET(v) => {}
            // RuntimeCommand::RESUME(v) => {}
            // RuntimeCommand::SYSCALL(v) => {}
            // RuntimeCommand::TROPHY(v) => {}
            // RuntimeCommand::UNLOCK(v) => {}
            RuntimeCommand::LAYERINIT(v) => v.start(context, vm_state, adv_state),
            RuntimeCommand::LAYERLOAD(v) => v.start(context, vm_state, adv_state),
            RuntimeCommand::LAYERUNLOAD(v) => v.start(context, vm_state, adv_state),
            RuntimeCommand::LAYERCTRL(v) => v.start(context, vm_state, adv_state),
            // RuntimeCommand::LAYERWAIT(v) => {}
            // RuntimeCommand::LAYERSWAP(v) => {}
            // RuntimeCommand::LAYERSELECT(v) => {}
            // RuntimeCommand::MOVIEWAIT(v) => {}
            // RuntimeCommand::TRANSSET(v) => {}
            // RuntimeCommand::TRANSWAIT(v) => {}
            // RuntimeCommand::PAGEBACK(v) => {}
            // RuntimeCommand::PLANESELECT(v) => {}
            // RuntimeCommand::PLANECLEAR(v) => {}
            // RuntimeCommand::MASKLOAD(v) => {}
            // RuntimeCommand::MASKUNLOAD(v) => {}
            // RuntimeCommand::CHARS(v) => {}
            // RuntimeCommand::TIPSGET(v) => {}
            // RuntimeCommand::QUIZ(v) => {}
            // RuntimeCommand::SHOWCHARS(v) => {}
            // RuntimeCommand::NOTIFYSET(v) => {}
            // RuntimeCommand::DEBUGOUT(v) => {}
            _ => todo!(),
        }
    }
}

pub enum CommandStartResult {
    /// Continue VM execution
    Continue(CommandResult),
    /// Yield to the game loop, run the command to completion, execution continued with the result
    Yield(ExecutingCommand),
    Exit,
}

impl From<CommandResult> for CommandStartResult {
    fn from(result: CommandResult) -> Self {
        CommandStartResult::Continue(result)
    }
}

impl From<ExecutingCommand> for CommandStartResult {
    fn from(command: ExecutingCommand) -> Self {
        CommandStartResult::Yield(command)
    }
}

pub trait StartableCommand {
    fn apply_state(&self, state: &mut VmState);
    fn start(
        self,
        context: &UpdateContext,
        vm_state: &VmState,
        adv_state: &mut AdvState,
    ) -> CommandStartResult;
}
