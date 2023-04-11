use crate::compressor::Compressor;

use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
use rocket::Data;
use rocket_multipart_form_data::{
    mime, multer, MultipartFormData, MultipartFormDataError, MultipartFormDataField,
    MultipartFormDataOptions,
};
use rocket_raw_response::RawResponse;

const MAX_DATA_BYTES: u64 = 33 * 1024 * 1024;

pub struct ImageCompressService {}

impl ImageCompressService {
    pub fn new() -> Self {
        ImageCompressService {}
    }

    pub async fn compress(
        &self,
        content_type: &ContentType,
        data: Data<'_>,
    ) -> Result<RawResponse, Custom<String>> {
        let options = MultipartFormDataOptions {
            max_data_bytes: MAX_DATA_BYTES,
            allowed_fields: vec![MultipartFormDataField::raw("image")
                .size_limit(MAX_DATA_BYTES)
                .content_type_by_string(Some(mime::IMAGE_STAR))
                .unwrap()],
            ..MultipartFormDataOptions::default()
        };

        let mut multipart_from_data =
            match MultipartFormData::parse(content_type, data, options).await {
                Ok(multipart_form_date) => multipart_form_date,
                Err(err) => match err {
                    MultipartFormDataError::DataTooLargeError(e) => {
                        return Err(Custom(Status::BadRequest, e.to_string()));
                    },
                    MultipartFormDataError::DataTypeError(_e) => {
                        return Err(Custom(
                            Status::BadRequest,
                            "This is not an image".to_string(),
                        ));
                    },
                    MultipartFormDataError::MulterError(multer::Error::IncompleteFieldData {
                        ..
                    })
                    | MultipartFormDataError::MulterError(multer::Error::IncompleteHeaders {
                        ..
                    }) => {
                        return Err(Custom(
                            Status::BadRequest,
                            "The Request Body Seems to be too large".to_string(),
                        ));
                    },
                    _ => {
                        panic!("{:?}", err);
                    },
                },
            };

        let image = multipart_from_data.raw.remove("image");

        match image {
            Some(mut image) => {
                let engine = Compressor::new();
                let raw_field = image.remove(0);
                let content_type = raw_field.content_type;
                let file_name = raw_field.file_name.unwrap_or_else(|| "Image".to_string());
                let data = raw_field.raw;
                let result = engine.compress_from_memory(&data);

                match result {
                    Ok(data) => Ok(RawResponse::from_vec(data, Some(file_name), content_type)),
                    Err(e) => Err(Custom(Status::BadRequest, e.to_string())),
                }
            },

            None => Err(Custom(Status::BadRequest, "No Image Provided".to_string())),
        }
    }
}
