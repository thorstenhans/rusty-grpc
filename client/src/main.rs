use std::io::stdin;

use voting::voting_client::VotingClient;
use voting::voting_request::Vote;
use voting::VotingRequest;

pub mod voting {
    tonic::include_proto!("voting");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = VotingClient::connect("http://[::1]:8080").await?;
    loop {
        println!("\nPlease vote for a particular url");
        let mut url = String::new();
        let mut vote = String::new();
        println!("Please provide a url: ");
        stdin().read_line(&mut url)?;
        println!("Please vote (d)own or (u)p: ");
        stdin().read_line(&mut vote)?;
        let vote = match vote.trim().to_lowercase().chars().next() {
            Some('u') => Vote::Up,
            Some('d') => Vote::Down,
            _ => break,
        };
        let request = tonic::Request::new(VotingRequest {
            url: url.trim().to_string(),
            vote: vote.into(),
        });
        let response = client.vote(request).await?;
        println!("Got: '{}' from service", response.into_inner().confirmation);
    }
    Ok(())
}
