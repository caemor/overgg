//! Upcoming Matches is 'parsing' the upcoming matches from over.gg
use select;
use select::predicate::{Class};
use helpers::{remove_clutter, getcontent};



/// Contains all usable data from the over.gg upcoming matches page
#[derive(Default, Debug, PartialEq)]
pub struct UpcomingMatch {
	///A link to the match page
	pub link: String,
	///Team 1
	pub team1: UpcomingTeam,
	///Team 2
	pub team2: UpcomingTeam,
	///Time remaining until the start of the match
    pub time: String,
    ///Name of the Event of the Game
    pub eventname: String,
    ///Name of the current round of the game like 'Group B Lower Final'
    pub eventsub: String,
}

/// Upcoming Team only gives a small amout of information about a team
/// like Teamname, Score if the game is already running and maybe a flag
#[derive(Default, Debug, PartialEq)]
pub struct UpcomingTeam {
	///Teamname
	pub name: String,
	///Flag of the country of the team (currently not parsed)
	pub flag: Option<String>,
	///Points in the current match if it is already running
	pub score: Option<u8>
}

/// Returns all upcoming matches on the first site from over.gg
pub fn get() -> Result<Vec<UpcomingMatch>, String> {
	matches(None)
}

/// Returns the first x upcoming matches from over.gg
pub fn get_x(number_of_matches: u32) -> Result<Vec<UpcomingMatch>, String> {
	matches(Some(number_of_matches))
}



fn matches(number_of_matches: Option<u32>) -> Result<Vec<UpcomingMatch>, String> {
    let document = getcontent("https://www.over.gg/matches")?;

    let mut matches: Vec<UpcomingMatch> = Vec::new();

    let mut event_date_storage = String::new();
    
    //Sets the limit to the amount of matches (if there is one)
    let mut counter = 0;
    let limit = number_of_matches.unwrap_or(0);

    //Goes through all upcoming matches
    for game in document.find(Class("match-item")) {
		
    	let (upc_match, event_date_storage_tmp) = parse_match(game, &event_date_storage)?;
    	event_date_storage = event_date_storage_tmp;
		matches.push(upc_match);

		counter += 1;
		if limit != 0 && counter >= limit {
			break;
		}

	}
	Ok(matches)
}
//TODO make test cases!
fn parse_match(game: select::node::Node, event_date_storage: &str) -> Result<(UpcomingMatch, String), String> {
	//
	let mut event_date_storage = event_date_storage.to_string();
	
	trace!("{:?}", remove_clutter(game.html()));
	let link = "https://over.gg".to_string() + game.attr("href").ok_or("Could not get link to the match")?;
	let eventname = game.find(Class("match-item-event-name")).next().ok_or("Could not get Eventname")?.text();
	let eventsub = game.find(Class("match-item-event-sub")).next().ok_or("Could not get Event subname")?.text();

	let mut event_date = game.find(Class("match-item-date")).next().ok_or("Could not get Eventdate")?.text().trim().to_string();
	
	if event_date == "" {
		event_date = event_date_storage.clone();
	} else {
		event_date_storage = event_date;
	}

	let event_time = game.find(Class("match-item-time")).next().ok_or("Could not get Event time")?.text();
	


	let mut one = true;



	let mut team1 = UpcomingTeam::default();
	let mut team2 = UpcomingTeam::default();

	for team in game.find(Class("match-item-vs-team")).take(2) {
		let name = team.find(Class("match-item-vs-team-name")).next().ok_or("Could not get team name")?.text();

		let score = team.find(Class("match-item-vs-team-score")).next().ok_or("Could not get team score")?.text();

		let name = remove_clutter(name);
		let score = remove_clutter(score);

		
		//Only already running games posses a score so it is converted to a option
		let score = score.parse::<u8>().ok();

		trace!("Team: {:?}, Score: {:?}, Unprepared Data: {:?}", name.clone(), score, team.clone());
		if one {
			team1 = UpcomingTeam{name, score, flag: None};				
			one = false;
		} else {
			team2 = UpcomingTeam{name, score, flag: None};	
		}
	}
	let eventtime = game.find(Class("match-item-eta")).next().ok_or("Could not get time remaining until event starts")?.text();

	let eventname = remove_clutter(eventname);
	let eventsub = remove_clutter(eventsub);
	let time = remove_clutter(eventtime.to_lowercase());



	let upc_match = UpcomingMatch {team1, team2, time, eventname, eventsub, link};
	

	Ok((upc_match, event_date_storage))

}


