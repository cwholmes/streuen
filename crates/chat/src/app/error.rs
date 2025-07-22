#[derive(Debug, thiserror::Error)]
pub enum ChatAppError {
    #[error(transparent)]
    NoiseError(#[from] libp2p::noise::Error),
    #[error(transparent)]
    BehaviorError(#[from] libp2p::BehaviourBuilderError),
    #[error(transparent)]
    SenderError(#[from] futures_channel::mpsc::SendError),
    #[error("Missing swarm sender.")]
    MissingSender,
}
