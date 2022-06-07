use personnel::AstronautJob;
use personnel::Candidate;

fn main() {
    let candidate = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Biologist),
        age: 21,
        health: 50,
    };

    println!("{}", job_score(&candidate));
    println!("{}", candidate_score(&candidate));
    ranking(Candidate::load_candidate_file());
}

fn job_score(c: &Candidate) -> u32 {
    let mut score = match c.primary_job {
        AstronautJob::Biogeochemist => 251,
        AstronautJob::Biologist => 257,
        AstronautJob::Engineer => 263,
        AstronautJob::Geologist => 269,
        AstronautJob::Mechanic => 271,
        AstronautJob::Medic => 277,
        AstronautJob::RoverOp => 281,
        AstronautJob::Scientist => 283,
    };

    match c.secondary_job {
        Some(AstronautJob::Biogeochemist) => score *= 251,
        Some(AstronautJob::Biologist) => score *= 257,
        Some(AstronautJob::Engineer) => score *= 263,
        Some(AstronautJob::Geologist) => score *= 269,
        Some(AstronautJob::Mechanic) => score *= 271,
        Some(AstronautJob::Medic) => score *= 277,
        Some(AstronautJob::RoverOp) => score *= 281,
        Some(AstronautJob::Scientist) => score *= 283,
        None => score *= score,
    }

    score % 577
}

fn candidate_score(c: &Candidate) -> u32 {
    let score = (job_score(c) + c.health as u32) * c.age as u32;

    if score > 3928 {
        score % 3929
    } else {
        score
    }
}

fn ranking(candidates: Vec<Candidate>) -> Vec<Candidate> {
    let mut candidates = candidates;

    candidates.sort_by(|a, b| candidate_score(b).cmp(&candidate_score(a)));

    candidates
}

#[test]
fn test_job_score_two_jobs() {
    let candidate = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Biologist),
        age: 21,
        health: 50,
    };

    assert_eq!(job_score(&candidate), 460);
}

#[test]
fn test_job_score_one_job() {
    let candidate = Candidate {
        primary_job: AstronautJob::Scientist,
        secondary_job: None,
        age: 21,
        health: 50,
    };

    assert_eq!(job_score(&candidate), 463);
}

#[test]
fn test_candidate_score_less_than() {
    let candidate = Candidate {
        primary_job: AstronautJob::Engineer,
        secondary_job: Some(AstronautJob::Geologist),
        age: 1,
        health: 88,
    };
    assert_eq!(candidate_score(&candidate), 441);
}

#[test]
fn test_candidate_score_greater_than() {
    let candidate = Candidate {
        primary_job: AstronautJob::RoverOp,
        secondary_job: Some(AstronautJob::Medic),
        age: 34,
        health: 88,
    };

    assert_eq!(candidate_score(&candidate), 993);
}

#[test]
fn test_candidate_ranking() {
    let a = Candidate {
        primary_job: AstronautJob::Engineer,
        secondary_job: Some(AstronautJob::Geologist),
        age: 8,
        health: 88,
    };

    let b = Candidate {
        primary_job: AstronautJob::RoverOp,
        secondary_job: Some(AstronautJob::Medic),
        age: 34,
        health: 88,
    };

    let c = Candidate {
        primary_job: AstronautJob::Mechanic,
        secondary_job: None,
        age: 12,
        health: 88,
    };

    let mut candidate = vec![c, b, a];
    candidate = ranking(candidate);

    assert_eq!(candidate_score(&candidate[0]), 3528);
}
