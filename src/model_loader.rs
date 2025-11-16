use tobj::Model;

pub fn load_monkey() -> Model {
    let (models, _) = tobj::load_obj(
        "assets/models/monkey.obj",
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(models.len(), 1, "Expected exactly one model in monkey.obj");
    models.get(0).expect("Expected monkey model").clone()
}

#[cfg(test)]
mod tests {
    use crate::model_loader::load_monkey;

    #[test]
    fn dummy_test() {
        let monkey_model = load_monkey();

        println!("Model name: {}", monkey_model.name);
        println!(
            "Loaded monkey model with {} positions",
            monkey_model.mesh.positions.len()
        );
        println!(
            "Loaded monkey model with {} indices",
            monkey_model.mesh.indices.len()
        );
        println!(
            "Loaded monkey model with {} normals",
            monkey_model.mesh.normals.len()
        );
        println!(
            "Loaded monkey model with {} texcoords",
            monkey_model.mesh.texcoords.len()
        );

        assert!(monkey_model.mesh.positions.len() > 0);
        assert!(monkey_model.mesh.normals.len() > 0);
        assert_eq!(
            monkey_model.mesh.positions.len(),
            monkey_model.mesh.normals.len()
        );
    }
}
