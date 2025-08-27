use tonic::Request;
use tonic::transport::{Endpoint}; 
use protobuf::proto;

mod grpc_pb {
    grpc::include_generated_proto!("generated", "routeguide");
}

use grpc_pb::{
    route_guide_client::RouteGuideClient,
    Point,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //ncreate end point to connect to
    let endpoint = Endpoint::new("http://[::1]:10000")?;
    let channel = endpoint.connect().await?;

    // create new client
    let mut client = RouteGuideClient::new(channel);

    println!("*** SIMPLE RPC ***");
    let point = proto!(Point{
        latitude: 409_146_138,
        longitude: -746_188_906
    });
    let response = client
        .get_feature(Request::new(point))
        .await?.into_inner();

    println!("Response = Name = \"{}\", Latitude = {}, Longitude = {}",
        response.name(),
        response.location().latitude(),
        response.location().longitude());
    Ok(())
}