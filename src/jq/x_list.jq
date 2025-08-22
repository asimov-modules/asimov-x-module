{
  "@context": {
    "know": "https://know.dev/",
    "xsd": "http://www.w3.org/2001/XMLSchema#",
    "count": {
      "@id": "know:count",
      "@type": "xsd:integer"
    },
    "members": {
      "@id": "know:members",
      "@type": "know:Collection"
    },
    "id": {
      "@id": "know:id",
      "@type": "xsd:string"
    },
    "name": {
      "@id": "know:name",
      "@language": "en"
    },
    "username": {
      "@id": "know:username",
      "@type": "xsd:string"
    },
    "description": {
      "@id": "know:description",
      "@language": "en"
    },
    "location": {
      "@id": "know:location",
      "@type": "xsd:string"
    },
    "profile_image_url": {
      "@id": "know:profileImageUrl",
      "@type": "@id"
    },
    "profile_banner_url": {
      "@id": "know:profileBannerUrl",
      "@type": "@id"
    },
    "verified": {
      "@id": "know:verified",
      "@type": "xsd:boolean"
    },
    "protected": {
      "@id": "know:protected",
      "@type": "xsd:boolean"
    },
    "created_at": {
      "@id": "know:createdAt",
      "@type": "xsd:dateTime"
    },
    "followers_count": {
      "@id": "know:followersCount",
      "@type": "xsd:integer"
    },
    "following_count": {
      "@id": "know:followingCount",
      "@type": "xsd:integer"
    },
    "tweet_count": {
      "@id": "know:tweetCount",
      "@type": "xsd:integer"
    },
    "listed_count": {
      "@id": "know:listedCount",
      "@type": "xsd:integer"
    }
  },
  "@id": "https://x.com/i/lists",
  "@type": ["know:XListMembers", "know:Collection"],
  "members": {
    "@type": "know:Collection",
    "count": ((.data // []) | length),
    "items": [
      (.data // [])[] | {
        "@type": "know:XUser",
        "id": .id,
        "name": .name,
        "username": .username,
        "description": (.description // ""),
        "location": (.location // ""),
        "profile_image_url": .profile_image_url,
        "profile_banner_url": .profile_banner_url,
        "verified": .verified,
        "protected": .protected,
        "created_at": .created_at,
        "followers_count": (.public_metrics.followers_count // null),
        "following_count": (.public_metrics.following_count // null),
        "tweet_count": (.public_metrics.tweet_count // null),
        "listed_count": (.public_metrics.listed_count // null)
      }
    ]
  }
}