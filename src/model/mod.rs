use burn::module::Module;
use burn::nn::{Linear, LinearConfig};
use burn::tensor::Tensor;

#[derive(Module, Debug)]
pub struct DevAdaptModel<B: burn::tensor::backend::Backend> {
    skill_head: Linear<B>,
    workflow_head: Linear<B>,
}

impl<B: burn::tensor::backend::Backend> DevAdaptModel<B> {
    pub fn new(
        device: &B::Device,
        input_dim: usize,
        skill_dim: usize,
        workflow_dim: usize,
    ) -> Self {
        let skill_head = LinearConfig::new(input_dim, skill_dim).init(device);
        let workflow_head = LinearConfig::new(input_dim, workflow_dim).init(device);
        Self {
            skill_head,
            workflow_head,
        }
    }

    pub fn forward(&self, features: Tensor<B, 2>) -> (Tensor<B, 2>, Tensor<B, 2>) {
        let skill_logits = self.skill_head.forward(features.clone());
        let workflow_logits = self.workflow_head.forward(features);
        (skill_logits, workflow_logits)
    }
}
