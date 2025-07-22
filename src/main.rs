mod models;

use models::activity::ActivitiesType;
use models::contact::Contact;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::io;

fn show_menu() {
    println!("What do you want to do?");
    println!("1. Add contact: add <name> <email> <phone> <city>");
    println!("2. Remove contact: remove <email>");
    println!("3. Show contacts: list");
    println!("4. Search contact by name: find <prefix>");
    println!("5. Show activity log: activity");
    println!("Command: ");
}

fn add_contact(
    new_contact: Contact,
    contacts: &mut HashMap<String, Contact>,
    sorted_contacts: &mut BTreeSet<String>,
    contacts_by_city: &mut HashMap<String, Vec<Contact>>,
    prefix_names: &mut BTreeMap<String, Contact>,
    phones_registered: &mut HashSet<String>,
) {
    let name: String = new_contact.name.clone();
    let email: String = new_contact.email.clone();
    let city: String = new_contact.city.clone();
    let phone: String = new_contact.phone.clone();
    if !phones_registered.insert(phone) {
        println!("The phone is already registered");
        return;
    }
    contacts.insert(email, new_contact.clone());
    sorted_contacts.insert(name.clone());
    contacts_by_city
        .entry(city)
        .or_insert(Vec::new())
        .push(new_contact.clone());
    prefix_names.insert(name.clone(), new_contact);
}

fn remove_contact(
    email: &String,
    contacts: &mut HashMap<String, Contact>,
    sorted_contacts: &mut BTreeSet<String>,
    contacts_by_city: &mut HashMap<String, Vec<Contact>>,
    _prefix_names: &mut BTreeMap<String, Contact>,
    phones_registered: &mut HashSet<String>,
) {
    if !contacts.get(email).is_some() {
        println!("No email found");
        return;
    }

    let contact: Contact = match contacts.get(email) {
        Some(c) => c.clone(),
        None => return
    };

    let name: String = contact.name.clone();
    let phone: String = contact.phone.clone();
    let city: String = contact.city.clone();

    phones_registered.retain(|p| p.to_string() != phone);

    sorted_contacts.remove(&name);
   
    if let Some(vc) = contacts_by_city.get_mut(&city) {
        vc.retain(|c| c.phone != contact.phone);
    }
}

fn main() {
    let mut contacts: HashMap<String, Contact> = HashMap::new();
    let mut sorted_contacts: BTreeSet<String> = BTreeSet::new();
    let mut contacts_by_city: HashMap<String, Vec<Contact>> = HashMap::new();
    let mut prefix_names: BTreeMap<String, Contact> = BTreeMap::new();
    let mut phones_registered: HashSet<String> = HashSet::new();
    let mut activity: VecDeque<ActivitiesType> = VecDeque::new();
    println!("This will be your agend!!!: ");
    show_menu();
    let mut command: String = String::new();
    io::stdin().read_line(&mut command).unwrap();
    loop {
        let parts: Vec<&str> = command.split_whitespace().collect::<Vec<&str>>();
        match parts.get(0) {
            Some(word) => match *word {
                "add" => {
                    let name: String = parts.get(1).copied().unwrap_or("unknown").to_string();
                    let email: String = parts.get(2).copied().unwrap_or("unknown").to_string();
                    let phone: String = parts.get(3).copied().unwrap_or("unknown").to_string();
                    let city: String = parts.get(4).copied().unwrap_or("unknown").to_string();
                    let new_contact: Contact = Contact::new(name, email, phone, city);
                    add_contact(
                        new_contact,
                        &mut contacts,
                        &mut sorted_contacts,
                        &mut contacts_by_city,
                        &mut prefix_names,
                        &mut phones_registered,
                    );
                    activity.push_back(ActivitiesType::ADD);
                },
                "remove" => {
                    let email: String = parts.get(1).copied().unwrap_or("unknown").to_string();
                    remove_contact(
                        &email,
                        &mut contacts,
                        &mut sorted_contacts,
                        &mut contacts_by_city,
                        &mut prefix_names,
                        &mut phones_registered,
                    )
                },  
                _ => break,
            },
            None => break,
        }
        command.clear();
        show_menu();
        io::stdin().read_line(&mut command).unwrap();
    }
}
