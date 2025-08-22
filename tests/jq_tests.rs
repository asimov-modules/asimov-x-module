// This is free and unencumbered software released into the public domain.

use asimov_x_module::jq::x_list;
use serde_json::json;

#[test]
fn test_x_list_jq_compilation() {
    let _filter = x_list();
}

#[test]
fn test_x_list_jq_with_sample_data() {
    let filter = x_list();
    let sample_data = json!({
        "data": [
            {
                "id": "123",
                "name": "Sample User",
                "username": "sample_user",
                "description": "Sample description",
                "location": "Sample Location",
                "profile_image_url": "https://example.com/image.jpg",
                "verified": false,
                "protected": false,
                "created_at": "2024-01-01T00:00:00Z",
                "public_metrics": {
                    "followers_count": 100,
                    "following_count": 50,
                    "tweet_count": 200,
                    "listed_count": 5
                }
            }
        ],
        "meta": {
            "result_count": 1
        }
    });

    let result = filter.filter_json(sample_data);
    assert!(result.is_ok());
}


