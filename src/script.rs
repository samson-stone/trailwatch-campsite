#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrailAction {
    pub module: String,
    pub family: String,
    pub shard: u32,
    pub survey: u32,
    pub seed: u32,
    pub limit: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TrailPlan {
    pub actions: Vec<TrailAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptError {
    InvalidLine(String),
    MissingField(&'static str),
    InvalidNumber(String),
}

impl TrailPlan {
    pub fn parse(text: &str) -> Result<Self, ScriptError> {
        let mut current = PendingAction::default();
        let mut actions = Vec::new();
        for raw_line in text.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line == "END" {
                actions.push(current.finish()?);
                current = PendingAction::default();
                continue;
            }
            let (key, value) = line.split_once(' ').ok_or_else(|| ScriptError::InvalidLine(line.to_string()))?;
            match key {
                "MODULE" => current.module = Some(value.trim().to_string()),
                "FAMILY" => current.family = Some(value.trim().to_string()),
                "SHARD" => current.shard = Some(parse_u32(value.trim())?),
                "RECORDS" => current.survey = Some(parse_u32(value.trim())?),
                "SEED" => current.seed = Some(parse_u32(value.trim())?),
                "LIMIT" => current.limit = Some(parse_usize(value.trim())?),
                _ => return Err(ScriptError::InvalidLine(line.to_string())),
            }
        }
        if current.has_data() {
            actions.push(current.finish()?);
        }
        Ok(Self { actions })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct PendingAction {
    module: Option<String>,
    family: Option<String>,
    shard: Option<u32>,
    survey: Option<u32>,
    seed: Option<u32>,
    limit: Option<usize>,
}

impl PendingAction {
    fn has_data(&self) -> bool {
        self.module.is_some()
            || self.family.is_some()
            || self.shard.is_some()
            || self.survey.is_some()
            || self.seed.is_some()
            || self.limit.is_some()
    }

    fn finish(self) -> Result<TrailAction, ScriptError> {
        Ok(TrailAction {
            module: self.module.ok_or(ScriptError::MissingField("MODULE"))?,
            family: self.family.ok_or(ScriptError::MissingField("FAMILY"))?,
            shard: self.shard.ok_or(ScriptError::MissingField("SHARD"))?,
            survey: self.survey.ok_or(ScriptError::MissingField("RECORDS"))?,
            seed: self.seed.unwrap_or(1),
            limit: self.limit.unwrap_or(5),
        })
    }
}

fn parse_u32(value: &str) -> Result<u32, ScriptError> {
    value.parse::<u32>().map_err(|_| ScriptError::InvalidNumber(value.to_string()))
}

fn parse_usize(value: &str) -> Result<usize, ScriptError> {
    value.parse::<usize>().map_err(|_| ScriptError::InvalidNumber(value.to_string()))
}