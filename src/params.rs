use std::str::FromStr;

use rocket::http::ContentType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageType {
    Best,
    PNG,
    JPEG,
    WEBP,
    GIF
}

impl Default for ImageType {
    fn default() -> Self {
        ImageType::Best
    }
}

impl FromStr for ImageType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "best" => Ok(ImageType::Best),
            "png" => Ok(ImageType::PNG),
            "jpeg" => Ok(ImageType::JPEG),
            "webp" => Ok(ImageType::WEBP),
            "gif" => Ok(ImageType::GIF),
            _ => Err(()),
        }
    }
}

impl Into<ContentType> for ImageType {
    fn into(self) -> ContentType {
        match self {
            ImageType::Best => ContentType::PNG,
            ImageType::PNG => ContentType::PNG,
            ImageType::JPEG => ContentType::JPEG,
            ImageType::WEBP => ContentType::WEBP,
            ImageType::GIF => ContentType::GIF,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProxyParameters<IT>
    where IT: FromStr + Clone + Copy
{
    pub resolution: Option<(u32, u32)>,
    pub file_type: IT,
}

impl FromStr for ProxyParameters<ImageType> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("/");
        let mut params = ProxyParameters::default();

        for part in parts.by_ref() {
            if is_resolution(part) {
                let res = part.split("x").collect::<Vec<&str>>();
                params.resolution = Some((res[0].parse().unwrap(), res[1].parse().unwrap()));
            } else {
                params.file_type = part.parse().unwrap();
            }
        }
        Ok(params)
    }
}

fn is_resolution(s: &str) -> bool {
    let parts = s.split("x").collect::<Vec<&str>>();
    if parts.len() != 2 {
        return false;
    }
    for part in parts {
        if let Ok(_) = part.parse::<u32>() {
            continue;
        }
        return false;
    }
    true
}