enum Titles {
    Maintenance,
    Marketing,
    Managers,
    Supervisor,
    Cook,
    Technician,
}
enum Status {
    Active,
    Terminated,
}
struct Employee {
    title: Titles,
    status: Status,
}
fn access(val: Employee) -> Result<(), String> {
    match val.status {
        Status::Terminated => Err("Denied access".to_string()),
        Status::Active => match val.title {
            Titles::Maintenance | Titles::Marketing | Titles::Managers => Ok(()),
            Titles::Supervisor | Titles::Cook | Titles::Technician => {
                Err("Denied access".to_string())
            }
        },
    }
}
fn print_result(val: Employee) -> Result<(), String> {
    let attempt_access = access(val)?;
    println!("Access granted {attempt_access:?}");
    Ok(())
}

fn main() {
    let mike = Employee {
        title: Titles::Cook,
        status: Status::Active,
    };
    match print_result(mike) {
        Err(e)=>println!("access denied : {e:?}"),
        _=>()
    }
}
