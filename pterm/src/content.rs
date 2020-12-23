static INTRO_LOGS: &str = include_str!("../../static/assets/files/logs.txt");
static INTRO_MSG: &str = include_str!("../../static/assets/files/intro.txt");
// static RESUME_CONTENT: &str = include_str!("../../static/assets/files/resume.txt");
static RANDOM_STRINGS: &str = include_str!("../../static/assets/files/files.txt");
static TMP_SOURCES: &str = include_str!("../../static/assets/files/sources.txt");
static GEORGE_PICS_FILENAMES: &str = include_str!("../../static/assets/files/george.txt");

/// resume content
static RESUME_AWARDS_FILE: &str = include_str!("../../static/assets/files/resume/awards.txt");
static RESUME_EDUCATION_FILE: &str = include_str!("../../static/assets/files/resume/education.txt");
static RESUME_EXPERIENCE_FILE: &str = include_str!("../../static/assets/files/resume/experience.txt");
static RESUME_LANGUAGES_FILE: &str = include_str!("../../static/assets/files/resume/languages.txt");
static RESUME_TECH_FILE: &str = include_str!("../../static/assets/files/resume/technologies.txt");

// TODO: publications 

lazy_static::lazy_static! {
  pub static ref INTRO_MSG_FULL: Vec<&'static str> = INTRO_MSG.lines().collect();
  pub static ref INTRO_LOGS_FULL: Vec<&'static str> = INTRO_LOGS.lines().collect();
  // pub static ref RESUME: Vec<&'static str> = RESUME_CONTENT.lines().collect();
  pub static ref RANDOM_STRINGS_LIST: Vec<&'static str> = RANDOM_STRINGS.lines().collect();
  pub static ref TMP_SOURCE_LIST: Vec<&'static str> = TMP_SOURCES.lines().collect();
  pub static ref GEORGE_PICS: Vec<&'static str> = GEORGE_PICS_FILENAMES.lines().collect();


  // resume
  pub static ref RESUME_AWARDS: Vec<&'static str> = RESUME_AWARDS_FILE.lines().collect();
  pub static ref RESUME_EDUCATION: Vec<&'static str> = RESUME_EDUCATION_FILE.lines().collect();
  pub static ref RESUME_EXPERIENCE: Vec<&'static str> = RESUME_EXPERIENCE_FILE.lines().collect();
  pub static ref RESUME_LANGUAGES: Vec<&'static str> = RESUME_LANGUAGES_FILE.lines().collect();
  pub static ref RESUME_TECH: Vec<&'static str> = RESUME_TECH_FILE.lines().collect();
}

pub static TMP_EXTENSIONS_LIST: &[&str] = &[
  "txt~", "sh~", "py~", "pdf~", "md~", "yaml~",
];

pub static EXTENSIONS_LIST: &[&str] = &[
  "gif", "webm", "mp4", "html", "php", "md", "png", "jpg", "ogg", "mp3", "flac", "iso", "zip",
  "rar", "tar.gz", "tar.bz2", "tar.xz", "deb", "rpm", "exe",
];