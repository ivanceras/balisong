//many more properties
//http://en.wikipedia.org/wiki/List_of_materials_properties

pub struct Material{
    opacity:u8,
    density:u8,//specific weight
    specularity:u8,
    refraction:u8,
    hardness:u8,
}

impl Material{
    pub fn new(&self, opacity:u8, specularity:u8, density:u8, refraction:u8, hardness:u8)->{
        Material{opacity:opacity, denisty:density, specularity:specularity, refraction:u8, hardness:u8}
    }
}