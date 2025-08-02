use mobster::*;

#[test]
fn test_mob_operations() {
    let mut mob1 = Mob::new(
        "The Corleones".to_string(),
        Boss::new("Vito Corleone", 50)
    );
    let mut mob2 = Mob::new(
        "The Tattaglias".to_string(),
        Boss::new("Philip Tattaglia", 45)
    );

    // Recruit members
    mob1.recruit(("Sonny", 30));
    mob1.recruit(("Michael", 25));
    mob2.recruit(("Bruno", 28));

    // Test combat score
    assert_eq!(mob1.combat_score(), 2); // 2 Associates = 2
    assert_eq!(mob2.combat_score(), 1); // 1 Associate = 1

    // Test attack
    mob1.attack(&mut mob2);
    assert_eq!(mob2.members.len(), 0); // mob2 should lose its youngest member

    // Test stealing
    mob1.wealth = 1000;
    mob2.wealth = 500;
    mob1.steal(&mut mob2, 300);
    assert_eq!(mob1.wealth, 1300);
    assert_eq!(mob2.wealth, 200);

    // Test city conquest
    mob1.conquer_city(&[&mob2], "New York".to_string());
    assert!(mob1.cities.contains("New York"));
}

#[test]
#[should_panic(expected = "Cannot promote an Underboss further!")]
fn test_promotion_panic() {
    let mut member = Member {
        role: Role::Underboss,
        age: 40,
    };
    member.get_promotion(); // This should panic
}