struct Student { //defines the struct
    first_name: String,
    last_name: String,
    id_number: u32,
    gpa: f32,
}

impl Student { //all functions for struct must go inside this impl
    //Constructs person
    fn new(first: &str, last: &str, id: u32, gPa: f32) -> Student {
        Student {
           first_name: first.to_string(),
           last_name: last.to_string(),
           id_number: id,
           gpa: gPa,
       } 
    }
    //Get full name
    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name)
    }
    //Set first name
    fn set_first_name(&mut self, first: &str) {
        self.first_name = first.to_string();
    }
    //Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
    //Set ID#
    fn set_id(&mut self, id: u32) {
        self.id_number = id;
    }
    //Set GPA
    fn set_gpa(&mut self, gpa: f32) {
        self.gpa = gpa;
    }
    //Return tuple of every element in struct
    fn to_tuple(self) -> (String, String, u32, f32) {
        return (self.first_name, self.last_name, self.id_number, self.gpa)
    }
}


pub fn run() {
    let mut a = Student::new("John", "Doe", 0123456, 3.75);
    println!("Person a's full name is {}", a.full_name());
    a.set_first_name("Will");
    a.set_last_name("Smith");
    a.set_id(6543210);
    a.set_gpa(3.65);
    println!("{:?}", a.to_tuple());
}