use tonic::{Request, Response, Status, transport::Server};
use voting::voting_request::Vote;
use voting::voting_server::{Voting, VotingServer};
use voting::{VotingRequest, VotingResponse};

pub mod voting {
    tonic::include_proto!("voting");
}

#[derive(Debug, Default)]
pub struct VotingService {}

#[tonic::async_trait]
impl Voting for VotingService {
    async fn vote(
        &self,
        request: Request<VotingRequest>,
    ) -> Result<Response<VotingResponse>, Status> {
        let r = request.into_inner();
        let confirmation = match r.vote() {
            Vote::Up => format!("Happy to confirm that you upvoted for {}", r.url),
            Vote::Down => format!("Confirmation that you downvoted for {}", r.url),
        };
        Ok(Response::new(VotingResponse { confirmation }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse()?;
    let voting_service = VotingService::default();

    println!("Voting server listening on {address}");
    Server::builder()
        .add_service(VotingServer::new(voting_service))
        .serve(address)
        .await?;
    Ok(())
}
