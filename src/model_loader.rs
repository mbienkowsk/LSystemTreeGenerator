use tobj::Model;

fn load_obj_file(path: &str) -> Model {
    let (models, _) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(models.len(), 1, "Expected exactly one model in {}", path);
    models.first().expect("Expected model").clone()
}

pub fn load_monkey() -> Model {
    load_obj_file("assets/models/monkey.obj")
}

pub fn load_cone() -> Model {
    load_obj_file("assets/models/cone.obj")
}

pub fn load_floor() -> Model {
    load_obj_file("assets/models/floor.obj")
}

#[cfg(test)]
mod tests {
    use crate::model_loader::load_monkey;

    #[test]
    fn monkey_loads_correctly() {
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

        assert!(!monkey_model.mesh.positions.is_empty());
        assert!(!monkey_model.mesh.normals.is_empty());
        assert_eq!(
            monkey_model.mesh.positions.len(),
            monkey_model.mesh.normals.len()
        );
    }

    #[test]
    fn cone_loads_correctly() {
        let cone_model = crate::model_loader::load_cone();

        println!("Model name: {}", cone_model.name);
        println!(
            "Loaded cone model with {} positions",
            cone_model.mesh.positions.len()
        );
        println!(
            "Loaded cone model with {} indices",
            cone_model.mesh.indices.len()
        );
        println!(
            "Loaded cone model with {} normals",
            cone_model.mesh.normals.len()
        );
        println!(
            "Loaded cone model with {} texcoords",
            cone_model.mesh.texcoords.len()
        );

        assert!(!cone_model.mesh.positions.is_empty());
        assert!(!cone_model.mesh.normals.is_empty());
        assert_eq!(
            cone_model.mesh.positions.len(),
            cone_model.mesh.normals.len()
        );
    }

    #[test]
    fn floor_loads_correctly() {
        let floor_model = crate::model_loader::load_floor();

        println!("Model name: {}", floor_model.name);
        println!(
            "Loaded floor model with {} positions",
            floor_model.mesh.positions.len()
        );
        println!(
            "Loaded floor model with {} indices",
            floor_model.mesh.indices.len()
        );
        println!(
            "Loaded floor model with {} normals",
            floor_model.mesh.normals.len()
        );
        println!(
            "Loaded floor model with {} texcoords",
            floor_model.mesh.texcoords.len()
        );

        assert!(!floor_model.mesh.positions.is_empty());
        assert!(!floor_model.mesh.normals.is_empty());
        assert_eq!(
            floor_model.mesh.positions.len(),
            floor_model.mesh.normals.len()
        );
    }
}
