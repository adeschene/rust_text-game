
use crate::helpers::print_from_file;

/// Represents a NPC, including name, monoloques,
/// and various flags relating to their quests.

pub struct Npc {
    pub name: String,
    pub has_been_met: bool,
    pub given_quest_item: bool,
    pub quest_done: bool,
    pub monologue_intro: String,
    pub monologue_neutral: String,
    pub monologue_ending: String,
    pub monologue_done: String,
    pub location: usize,
}

/// Implementation of Npc struct.

impl Npc {

    /// Create a new Npc, setting all bools in the object to false.

    pub fn new(
        name: String,
        monologue_intro: String,
        monologue_neutral: String,
        monologue_ending: String,
        monologue_done: String,
        location: usize) -> Npc
    {
        Npc {
            name,
            has_been_met: false,
            given_quest_item: false,
            quest_done: false,
            monologue_intro,
            monologue_neutral,
            monologue_ending,
            monologue_done,
            location,
        }
    }

    /// Meant to create a new Npc that is an updated version
    /// of an existing Npc.

    pub fn update(
        name_ref: &str,
        has_been_met: bool,
        given_quest_item: bool,
        quest_done: bool,
        intro_ref: &str,
        neutral_ref: &str,
        ending_ref: &str,
        done_ref: &str,
        location: usize) -> Npc
    {
        Npc {
            name: name_ref.to_string(),
            has_been_met,
            given_quest_item,
            quest_done,
            monologue_intro: intro_ref.to_string(),
            monologue_neutral: neutral_ref.to_string(),
            monologue_ending: ending_ref.to_string(),
            monologue_done: done_ref.to_string(),
            location,
        }
    }

    /// Take a string representing a saved Npc and create an Npc
    /// object from its contents.

    pub fn deserialize(data: &str) -> Npc
    {
        // First entry always \n, so we skip it.
        let mut entries = data.lines().skip(1);
        Npc {
            name: entries.next().unwrap().parse().unwrap(),
            has_been_met: entries.next().unwrap().parse().unwrap(),
            given_quest_item: entries.next().unwrap().parse().unwrap(),
            quest_done: entries.next().unwrap().parse().unwrap(),
            monologue_intro: entries.next().unwrap().parse().unwrap(),
            monologue_neutral: entries.next().unwrap().parse().unwrap(),
            monologue_ending: entries.next().unwrap().parse().unwrap(),
            monologue_done: entries.next().unwrap().parse().unwrap(),
            location: entries.next().unwrap().parse().unwrap()
        }
    }

    /// Convert an Npc object into a String representation for saving.

    pub fn serialize(&self) -> String {
        let s = format!(
            // The individual Npc delimiter is '^'
            "\n^\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}"
            , &self.name.to_string()
            , &self.has_been_met.to_string()
            , &self.given_quest_item.to_string()
            , &self.quest_done.to_string()
            , &self.monologue_intro.to_string()
            , &self.monologue_neutral.to_string()
            , &self.monologue_ending.to_string()
            , &self.monologue_done.to_string()
            , &self.location.to_string()
                       );
        s
    }

    /// Sets the has_been_met field to true.
    /// Tracks whether the player has talked to the Npc at least once.

    pub fn meet_player(&self) -> Npc {
        Npc::update(
            &self.name,
            true,
            self.given_quest_item,
            self.quest_done,
            &self.monologue_intro,
            &self.monologue_neutral,
            &self.monologue_ending,
            &self.monologue_done,
            self.location,
        )
    }

    /// Sets the given_quest_item field to true.
    /// Tracks whether the player has talked to the Npc
    /// while holding their quest item, which effectively
    /// "gives" it to them.

    pub fn receive_item(&self) -> Npc {
        Npc::update(
            &self.name,
            self.has_been_met,
            true,
            self.quest_done,
            &self.monologue_intro,
            &self.monologue_neutral,
            &self.monologue_ending,
            &self.monologue_done,
            self.location,
        )
    }

    /// Sets the quest_done field to true.
    /// Tracks whether the player has talked to the Npc again
    /// after having given them their quest item, which gives
    /// the player the final piece of dialogue and end the
    /// quest officially.

    pub fn end_quest(&self) -> Npc {
        Npc::update(
            &self.name,
            self.has_been_met,
            self.given_quest_item,
            true,
            &self.monologue_intro,
            &self.monologue_neutral,
            &self.monologue_ending,
            &self.monologue_done,
            self.location,
        )
    }

    /// When the player talks to an Npc, this function is called
    /// and, based off of various boolean fields in the Npc object,
    /// displays 1 of 4 possible monologues, then updates said booleans
    /// so that a different monologue might be displayed next time.

    pub fn speak(&self) -> Npc {
        // Storyline completed.
        if self.quest_done {
            print_from_file(&self.monologue_done);
            self.end_quest() // Hack
        // Storyline almost done, but not quite.
        } else if self.given_quest_item {
            print_from_file(&self.monologue_ending);
            self.end_quest()
        // Storyline just started.
        } else if self.has_been_met {
            print_from_file(&self.monologue_neutral);
            self.meet_player() // Hackity hack
        } else { // First interaction.
            print_from_file(&self.monologue_intro);
            self.meet_player()
        }
    }
}

/// Creates a Vec of predetermined, somewhat hardcoded Npc objects.

pub fn generate_npcs() -> Vec<Npc> {
    vec![
        Npc::new(
                "Carl".to_string(),
                "../data/npc/carl/carltalk0.txt".to_string(),
                "../data/npc/carl/carltalk1.txt".to_string(),
                "../data/npc/carl/carltalk2.txt".to_string(),
                "../data/npc/carl/carltalk3.txt".to_string(),
                2,
        ),
        Npc::new(
                "Blimpo".to_string(),
                "../data/npc/blimpo/blimpotalk0.txt".to_string(),
                "../data/npc/blimpo/blimpotalk1.txt".to_string(),
                "../data/npc/blimpo/blimpotalk2.txt".to_string(),
                "../data/npc/blimpo/blimpotalk3.txt".to_string(),
                6,
        ),
    /* TODO: More NPCs

        Npc::new
            (
                "Chadrick".to_string(),
                "../data/npc/chad/chadtalk0.txt".to_string(),
                "../data/npc/chad/chadtalk1.txt".to_string(),
                "../data/npc/chad/chadtalk2.txt".to_string(),
                "../data/npc/chad/chadtalk3.txt".to_string(),
                8,
            ),
        Npc::new
            (
                "Puskinteo".to_string(),
                "../data/npc/pusk/puskinteotalk0.txt".to_string(),
                "../data/npc/pusk/puskinteotalk1.txt".to_string(),
                "../data/npc/pusk/puskinteotalk2.txt".to_string(),
                "../data/npc/pusk/puskinteotalk3.txt".to_string(),
                6,
            ),
    */
        ]
}

/// Loads each NPC's data and returns them in a Vec.

pub fn deserialize(data: &str) -> Vec<Npc> {
    // Npc object data in the save file is separated by '^'s
    // First will always be newline, so skip it.
    let npc_data = data.split("^").skip(1);
    let mut npcs = Vec::new();
    for npc in npc_data {
        // Don't try to load an empty string.
        if npc.is_empty() == false {
            // Load and push an NPC onto the vec.
            npcs.push(Npc::deserialize(npc));
        }
    }
    npcs
}

/// Converts each Npc in an Vec<Npc> into a String representation for saving.

pub fn serialize(npcs: Vec<Npc>) -> String {
    let mut npc_data = String::new();
    for npc in npcs { npc_data.push_str(&npc.serialize()) };
    npc_data
}
