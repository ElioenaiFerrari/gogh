use candle_core::Device;
use stable_diffusion::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;
    let weights = StableDiffusionWeights::new(StableDiffusionVersion::XL, DType::F32);
    let parameters = StableDiffusionParameters::new(weights, device, DType::F16)?;
    let stable_diffusion = StableDiffusion::new(parameters)?;
    let args = GenerationParameters::new("A green apple");
    let image = stable_diffusion.generate(args)?;
    image.save("output.png")?;
    Ok(())
}
