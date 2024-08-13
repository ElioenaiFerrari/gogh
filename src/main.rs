use std::io::Read;

use candle_core::Device;
use pb::gogh_server::Gogh;
use stable_diffusion::*;

mod pb {
    tonic::include_proto!("gogh.v1");
}

#[derive(Default)]
struct GoghService {
    // stable_diffusion: StableDiffusion,
}

#[tonic::async_trait]
impl Gogh for GoghService {
    async fn generate_image(
        &self,
        request: tonic::Request<pb::GenerateImageRequest>,
    ) -> Result<tonic::Response<pb::GenerateImageResponse>, tonic::Status> {
        let prompt = request.into_inner().prompt;

        if prompt.is_empty() {
            return Err(tonic::Status::invalid_argument("Prompt cannot be empty"));
        }

        let device = Device::Cpu;
        let weights = StableDiffusionWeights::new(StableDiffusionVersion::V1_5, DType::F32);
        let parameters = StableDiffusionParameters::new(weights, device, DType::F16).unwrap();
        let stable_diffusion = StableDiffusion::new(parameters).unwrap();
        let args = GenerationParameters::new(prompt);
        let id = uuid::Uuid::now_v7().to_string();
        let image = stable_diffusion
            .generate(args)
            .expect("Failed to generate image");

        image
            .save(format!("img/{id}.png"))
            .expect("Failed to save image");

        Ok(tonic::Response::new(pb::GenerateImageResponse {
            id,
            image: image.to_vec(),
        }))
    }

    async fn get_image(
        &self,
        request: tonic::Request<pb::GetImageRequest>,
    ) -> Result<tonic::Response<pb::GetImageResponse>, tonic::Status> {
        let id = request.into_inner().id;
        let mut file = std::fs::File::open(format!("img/{id}.png")).unwrap();
        let mut image = Vec::new();
        file.read_to_end(&mut image).unwrap();

        return Ok(tonic::Response::new(pb::GetImageResponse { image }));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let svc = pb::gogh_server::GoghServer::new(GoghService::default());
    let addr = "0.0.0.0:8080".parse()?;

    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}
