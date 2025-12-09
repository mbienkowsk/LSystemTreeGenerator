use crate::common::ModelSelection;
use tobj::{Material, Model};

#[derive(Clone, Debug)]
pub struct Model3D {
    pub geometry: Model,
    pub material: Material,
}

/// Load a 3D model from .obj and .mtl files containing one model and one material each.
fn load_obj_file(path: &str) -> Model3D {
    let (models, materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(models.len(), 1, "Expected exactly one model in {path}");
    let materials = materials.expect("Expected material file");
    assert_eq!(
        materials.len(),
        1,
        "Expected exactly one material in {path}"
    );

    let geometry = models.first().expect("Expected model").clone();
    let material = materials.first().expect("Expected material").clone();

    Model3D { geometry, material }
}

pub fn load_floor() -> Model3D {
    load_obj_file("assets/models/floor.obj")
}

fn load_monkey() -> Model3D {
    load_obj_file("assets/models/monkey.obj")
}

fn load_cylinder() -> Model3D {
    load_obj_file("assets/models/cylinder.obj")
}

fn load_branch() -> Model3D {
    load_obj_file("assets/models/branch.obj")
}

fn load_leaf() -> Model3D {
    load_obj_file("assets/models/leaf.obj")
}

fn load_twig() -> Model3D {
    load_obj_file("assets/models/twig.obj")
}

pub fn load_model(selected_model: ModelSelection) -> Model3D {
    match selected_model {
        ModelSelection::Cylinder => load_cylinder(),
        ModelSelection::Branch => load_branch(),
        ModelSelection::Leaf => load_leaf(),
        ModelSelection::Twig => load_twig(),
        ModelSelection::Monkey => load_monkey(),
    }
}

#[cfg(test)]
mod tests {
    use crate::model_loader::*;

    fn display_model_info(model: &Model3D) {
        println!("Model name: {}", model.geometry.name);
        println!(
            "Loaded model with {} positions",
            model.geometry.mesh.positions.len()
        );
        println!(
            "Loaded model with {} indices",
            model.geometry.mesh.indices.len()
        );
        println!(
            "Loaded model with {} normals",
            model.geometry.mesh.normals.len()
        );
        println!(
            "Loaded model with {} texcoords",
            model.geometry.mesh.texcoords.len()
        );
        println!("Loaded material name: {}", model.material.name);
        println!("Material ambient color: {:?}", model.material.ambient);
        println!("Material diffuse color: {:?}", model.material.diffuse);
        println!("Material specular color: {:?}", model.material.specular);
    }

    fn check_if_model_loaded_correctly(model: &Model3D) {
        assert!(!model.geometry.mesh.positions.is_empty());
        assert!(!model.geometry.mesh.normals.is_empty());
        assert_eq!(
            model.geometry.mesh.positions.len(),
            model.geometry.mesh.normals.len()
        );
    }

    #[test]
    fn monkey_loads_correctly() {
        let monkey_model = load_monkey();
        display_model_info(&monkey_model);
        check_if_model_loaded_correctly(&monkey_model);
    }

    #[test]
    fn cylinder_loads_correctly() {
        let model = load_cylinder();
        check_if_model_loaded_correctly(&model);
        display_model_info(&model);
    }

    #[test]
    fn branch_loads_correctly() {
        let model = load_branch();
        check_if_model_loaded_correctly(&model);
        display_model_info(&model);
    }

    #[test]
    fn leaf_loads_correctly() {
        let model = load_leaf();
        check_if_model_loaded_correctly(&model);
        display_model_info(&model);
    }

    #[test]
    fn twig_loads_correctly() {
        let model = load_twig();
        check_if_model_loaded_correctly(&model);
        display_model_info(&model);
    }
}
