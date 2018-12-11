//make student structure
struct Student
{
    name: String,
    studentid: u32,
    grade: f32,

}
//make class structure

struct Class
{
    name: String,
    coursecode: String,
    students: Vec<Student>
}

fn newClass(name: String, coursecode:String, students:Vec <Student>)->Class
{
    Class {
    name:name,
    coursecode:coursecode,
    students:students,
    }
}

fn printclass (x:&Class)
{
    let avg = average(&x.students);
    let pass=passing(&x.students);
    println!("{} - {} - Class Average: {}% Percentage of Passing Students: {}%", x.name, x.coursecode, avg, pass);
}

fn average(students:&Vec<Student>)->f32
{
    let mut n=0.0;
    let mut t=0.0;
  for x in students
  {
      n=n+1.0;
      t=t+x.grade;
  }
  t/n
}

fn passing(students:&Vec<Student>)->f32
{
    let mut n=0.0;
    let mut t=0.0;
    for x in students
    {
        printstudent(&x);
        if x.grade>50.0
        {
          n=n+1.0;
        }
        t=t+1.0
    }
    println!("There are {} passing students, and {} students total.", &n, &t);
    n/t
}
fn printstudent (x:&Student)
{
    println!("{} - {}: {}%", x.studentid, x.name, x.grade);
}


fn newStudent(name: String, id:u32, mark: f32)->Student
{
    Student {
    name:name,
    studentid:id,
    grade:mark
    }
}


fn main()
{
    let mut kids:Vec<Student>=Vec::new();
    let mut s1= newStudent(String::from("Jeremy"), 101, 86.2);
    let mut s2 = newStudent(String::from("Tom"), 102, 1.0 );
    let mut s3 = newStudent(String::from("Bob"), 103, 2.0 );

    //printstudent(&s1);
    //printstudent(&s2);
    //printstudent(&s3);

    kids.push(s1);
    kids.push(s2);
    kids.push(s3);
    let mut c1=newClass(String::from("Comp Sci"), String::from("CSCI 101U"), kids);
    //let mut avg:f32=average(c1.students);
    printclass(&c1);
//    println("Average is {}", kids.average());

}
