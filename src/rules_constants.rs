// Copyright D. Evans (djve60+kolbot@gmail.com).

// A collection of constants for the non-combat rules in the clan
// basement areas.

// Need to "use crate::rules_constants".
// Refrenced with "rules_constants::<NAME>".

use std::collections::HashMap;

// Crates from rust.io.
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ALL_RULES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();

        // Need help?
        m.insert("Help",
                 "rule <non-combat title> where the title is the adventure name, such as \"Mind Yer Binder\". This command is available as a private message (for all adventures), or in the appropriate clan channel.");

        // The SlimeTube.
        m.insert("Engulfed!",
                 "Either tickle the uvula or squirm back out.");

        // Hobopolis sewers."
        m.insert("Disgustin' Junction",
                 "Open the grate if fewer than 20 grates are opened.");
        m.insert("Somewhat Higher and Mostly Dry",
                 "Turn the valve if the water level has been lowered fewer than 20 times.");
        m.insert("The Former or the Ladder",
                 "If someone is in the cage, go down the ladder. You must check that nobody is in the cage before proceeding.");
        m.insert("Despite All Your Rage",
                 "Do not gnaw through the cage until you have asked to be freed in /hobopolis and /clan.");

        // Hobopolis Town Square.
        m.insert("Mind Yer Binder",
                 "Buy the binder if you have 30 nickels.");
        m.insert("Marketplace Entrance",
                 "You'll have a hard time navigating without any hobo codes.");
        m.insert("Attention -- A Tent!",
                 "Leave, unless you have your instrument and want to play (in the back). If someone is on stage, you may mosh (kill hobos) or busk (get nickels) in the front.");
        m.insert("Richard's Redoubt",
                 "You can build scarecrows if you'd like, but keep in mind it closes Hobopolis more quickly.");
        m.insert("Enter the Hoboverlord",
                 "Run away and organize in /hobopolis to fight Hodgman after all of the other bosses are dead.");

        // Hobopolis Burnbarrel Blvd.
        m.insert("Getting Tired",
                 "Put a tire on gently unless the stack is sufficiently high, or most of the hobos have been killed.");
        m.insert("Hot Dog! I Mean... Door!",
                 "Choose any option, but the door won't open unless water or steam has been diverted.");
        m.insert("Piping Hot",
                 "Leave the valve alone.");
        m.insert("Home, Home in the Range",
                 "Get out of the kitchen and organize in /hobopolis to fight Ol' Scratch.");

        // Exposure Esplanade.
        m.insert("The Frigid Air",
                 "You may pry open the freezer up to two times per run. After that you must pry yourself from the situation. Please donate if you get a frozen banquet.");
        m.insert("Piping Cold",
                 "The second valve is recommended, but you may turn whichever you wish.");
        m.insert("There Goes Fritz",
                 "Choose any option.");
        m.insert("Bumpity Bump Bump",
                 "Retreat and organize in /hobopolis to fight Frosty.");

        // The Heap.
        m.insert("The Compostal Service",
                 "Begone.");
        m.insert("You vs. The Volcano",
                 "Kick the volcano, unless people are waiting at I Refuse and you are sure there is already support for everyone.");
        m.insert("I Refuse!",
                 "Announce that you have arrived, and MAKE ABSOLUTELY SURE that everybody who is interested has arrived before exploring. DO NOT go AFK.");
        m.insert("Deep Enough to Dive",
                 "Do like the sign says and organize in /hobopolis to fight Oscus.");

        // The Ancient Hobo Burial Ground.
        m.insert("Returning to the Tomb",
                 "Choose any option.");
        m.insert("Ah, So That's Where They've All Gone",
                 "Tiptoe through the tulips and leave.");
        m.insert("A Chiller Night",
                 "Study the hobos' dance moves three times, then dance with them.");
        m.insert("Welcome To You!",
                 "Avoid this particular brush with death and organize in /hobopolis to fight Zombo.");

        // The Purple Light District.
        m.insert("The Furtivity of My City",
                 "If the Heap is not complete, intimidate. Otherwise, choose either other option.");
        m.insert("Getting Clubbed",
                 "Flimflam the crowd, unless the club is open. If you're going to nosepick, you need to flimflam an extra time before each nosepick.");
        m.insert("Exclusive!",
                 "Choose any option, but you must flimflam an extra time beforehand if you want to nosepick.");
        m.insert("Van, Damn",
                 "Don't bother knockin'. Instead, organize in /hobopolis to fight Chester.");

        // Dreadsylvanian Woods
        m.insert("The Cabin in the Dreadsylvanian Woods",
                 "Choose any option.  The attic may be locked.  This is a top level choice, ask again about the choice that follows.");
        m.insert("The Kitchen in the Woods",
                 "Do NOT clean out the disposal.");
        m.insert("What Lies Beneath (the Cabin)",
                 "Choose any option.  Newspapers give FKs, and the lockbox holds a unique item.");
        m.insert("Where it's Attic",
                 "Do NOT check out the wolfsbane or poke around in the rafters.  Only turn off the music box if you are an Accordion Thief (unique item).");
        m.insert("The Tallest Tree in the Forest",
                 "Choose any option.  Fire tower may be locked.  This is a top level choice, ask again about the choice that follows.");
        m.insert("Top of the Tree, Ma!",
                 "Do NOT kick the nest.  Only stomp on the branch IF a clannie is looking up from the base.  The shiny thing is unique.");
        m.insert("All Along the Watchtower",
                 "Do NOT wind up the siren.  The footlocker gives FKs.");
        m.insert("Treebasing",
                 "Choose any option.  Coordinate with a muscle class clannie if you want a blood kiwi (unique item).");
        m.insert("Staring Upwards...",
                 "Keep staring upwards until your clannie stomps and you get the blood kiwi (unique item).  Do NOT stop staring without making sure nobody's about to stomp.");
        m.insert("Below the Roots",
                 "Choose any option.  This is a top level choice, ask again about the choice that follows.");
        m.insert("Hot Coals",
                 "Do NOT pull the cork.");
        m.insert("The Heart of the Matter",
                 "Do NOT read the heart a story.");
        m.insert("Once Midden, Twice Shy",
                 "Do NOT smash the eggs.  The garbage gives FKs.");

        // Dreadsylvanian Village.
        m.insert("Dreadsylvanian Village Square",
                 "Choose any option.  The schoolhouse may be locked.  This is a top level choice, ask again about the choice that follows.");
        m.insert("Fright School",
                 "Do NOT clean the erasers.  Only take a pencil from the teacher's desk if you still need to permanently mark locations.");
        m.insert("Smith, Black as Night",
                 "Do NOT stoke the furnace.  The till gives FKs.");
        m.insert("Try New Extra-Strength Anvil",
                 "Choose any option.");
        m.insert("Good Noose, Everyone!",
                 "Do NOT paint the noose pink.  Coordinate with a clannie if you want to get the unique item from standing on the trap door.");
        m.insert("Hello, Gallows",
                 "Keep standing while waiting for a clannie to pull the lever.");
        m.insert("The Even More Dreadful Part of Town",
                 "Choose any option.  This is a top level choice, ask again about the choice that follows.");
        m.insert("A Dreadful Smell",
                 "Do NOT unclog the grate.");
        m.insert("Eight, Nine, Tenement",
                 "Do NOT drive out the rats or paint over the graffiti.");
        m.insert("The Tinker's. Damn.",
                 "Choose any option.  The shelves give FKs.");
        m.insert("The Old Duke's Estate",
                 "Choose any option.  The suite may be locked.  This is a top level choice, ask again about the choice that follows.");
        m.insert("The Plot Thickens",
                 "Do NOT close the gates.  Robbing the graves gives FKs.");
        m.insert("No Quarter",
                 "Do NOT turn off the ovens.");
        m.insert("The Master Suite -- Sweet!",
                 "Do NOT blow the whisle.");

        // Dreadsylvanian Castle
        m.insert("Tower Most Tall",
                 "Choose any option.  The laboratory may be locked.  This is a top level choice, ask again about the choice that follows.");
        m.insert("Working in the Lab, Late One Night",
                 "Do NOT turn on the lamp or examine the brains.  The machine takes a skull capacitor, so be sure to coordinate with clannies to use all charges, ahead of time.");
        m.insert("The Machine",
                 "Follow instructions from the machine fixer on when and where to get in.");
        m.insert("Among the Quaint and Curious Tomes",
                 "Do NOT read about bone-dissolving.");
        m.insert("In The Boudoir",
                 "Do NOT shut up the parrot.  The dresser gives FKs.");
        m.insert("This Hall is Really Great",
                 "Choose any option.  The ballroom may be locked.  This is a top level choice, ask again about the choice that follows.");
        m.insert("The Belle of the Ballroom",
                 "Do NOT play the organ.");
        m.insert("Cold Storage",
                 "Do NOT turn down the freezer.");
        m.insert("Dining In (the Castle)",
                 "Do NOT clear the dishes.  Grabbing the roast or levitating up to the rafters give unique items.");
        m.insert("The Dreadsylvanian Dungeon",
                 "Choose any option.  This is a top level choice, ask again about the choice that follows.");
        m.insert("Live from Dungeon Prison",
                 "Do NOT flush the toilet.  Here.  In THIS non-combat.  Please flush everywhere else!");
        m.insert("The Hot Bowels",
                 "Do NOT let off some steam.  The incinerator gives FKs.");
        m.insert("Among the Fungus",
                 "Choose any option.  Breaking off some choice bits gives a unique item.");
        m.insert("End of the Path",
                 "Clear out and organize in /dread to fight the woods boss.  Hard Mode kills should get priority, but don't let the boss linger.");
        m.insert("You're About to Fight City Hall",
                 "You can't fight city hall, so organize in /dread to fight the village boss.  Hard Mode kills should get priority, but don't let the boss linger.");
        m.insert("Holding Court",
                 "Begone and organize in /dread to fight the castle boss.  Hard Mode kills should get priority, but don't let the boss linger.");
        m
    };
}


lazy_static! {
    pub static ref SLIMETUBE_RULES: [&'static str; 2] =
        [ "Engulfed!", 
          "Help" ];
}

lazy_static! {
    pub static ref HOBOPOLIS_RULES: [&'static str; 30] =
        [ "A Chiller Night",
          "Ah, So That's Where They've All Gone",
          "Attention -- A Tent!",
          "Bumpity Bump Bump",
          "Deep Enough to Dive",
          "Despite All Your Rage",
          "Disgustin' Junction",
          "Enter the Hoboverlord",
          "Exclusive!",
          "Getting Clubbed",
          "Getting Tired",
          "Help",
          "Home, Home in the Range",
          "Hot Dog! I Mean... Door!",
          "I Refuse!",
          "Marketplace Entrance",
          "Mind Yer Binder",
          "Piping Cold",
          "Piping Hot",
          "Returning to the Tomb",
          "Richard's Redoubt",
          "Somewhat Higher and Mostly Dry",
          "The Compostal Service",
          "The Former or the Ladder",
          "The Frigid Air",
          "The Furtivity of My City",
          "There Goes Fritz",
          "Van, Damn",
          "Welcome To You!",
          "You vs. The Volcano" ];
}

lazy_static! {
    pub static ref DREADSYLVANIA_RULES: [&'static str; 44] =
        [ "A Dreadful Smell",
          "All Along the Watchtower",
          "Among the Fungus",
          "Among the Quaint and Curious Tomes",
          "Below the Roots",
          "Cold Storage",
          "Dining In (the Castle)",
          "Dreadsylvanian Village Square",
          "Eight, Nine, Tenement",
          "End of the Path",
          "Fright School",
          "Good Noose, Everyone!",
          "Hello, Gallows",
          "Help",
          "Holding Court",
          "Hot Coals",
          "In The Boudoir",
          "Live from Dungeon Prison",
          "No Quarter",
          "Once Midden, Twice Shy",
          "Smith, Black as Night",
          "Staring Upwards...",
          "The Belle of the Ballroom",
          "The Cabin in the Dreadsylvanian Woods",
          "The Dreadsylvanian Dungeon",
          "The Even More Dreadful Part of Town",
          "The Heart of the Matter",
          "The Hot Bowels",
          "The Kitchen in the Woods",
          "The Machine",
          "The Master Suite -- Sweet!",
          "The Old Duke's Estate",
          "The Plot Thickens",
          "The Tallest Tree in the Forest",
          "The Tinker's. Damn.",
          "This Hall is Really Great",
          "Top of the Tree, Ma!",
          "Tower Most Tall",
          "Treebasing",
          "Try New Extra-Strength Anvil",
          "What Lies Beneath (the Cabin)",
          "Where it's Attic",
          "Working in the Lab, Late One Night",
          "You're About to Fight City Hall" ];
}
