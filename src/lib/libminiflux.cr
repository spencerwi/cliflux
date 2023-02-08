require "http/client"
require "json"

module LibMiniflux
  class Client
    def initialize(url : String, @api_key : String)
      @url = URI.parse(url)
    end

    def get_unread_entries(limit : Int32, offset : Int32) : Array(FeedEntry)
      params = URI::Params.encode({
        "limit" => limit.to_s,
        "offset" => offset.to_s,
        "status" => "unread",
        "direction" => "desc",
        "order" => "published_at"
      })
      url = @url.dup
      url.query = params
      url.path = "/v1/entries"
      response = HTTP::Client.get(url, headers: self.get_auth_header())

      if (response.status_code >= 200 && response.status_code < 300)
        return FeedEntriesResponse.from_json(response.body).entries
      else
        raise "Error #{response.status_code}: #{response.body}"
      end
    end

    def mark_as_read(entry : FeedEntry)
        url = @url.dup
        url.path = "/v1/entries"
        response = HTTP::Client.put(url, headers: self.get_auth_header(), body: {
          "entry_ids" => [entry.id],
          "status" => "read"
        })
        if response.status_code != 204
          raise "Error #{response.status_code}: #{response.body}"
        end
    end

    def mark_as_unread(entry : FeedEntry)
        url = @url.dup
        url.path = "/v1/entries"
        response = HTTP::Client.put(url, headers: self.get_auth_header(), body: {
          "entry_ids" => [entry.id],
          "status" => "unread"
        })
        if response.status_code != 204
          raise "Error #{response.status_code}: #{response.body}"
        end
    end

    private def get_auth_header()
      HTTP::Headers {
        "X-Auth-Token" => @api_key
      }
    end
  end

  class FeedCategory
    include JSON::Serializable

    @[JSON::Field(key: "id")]
    property id : Int32

    @[JSON::Field(key: "user_id")]
    property user_id : Int32

    @[JSON::Field(key: "title")]
    property title : String
  end

  class Feed
    include JSON::Serializable

    @[JSON::Field(key: "id")]
    property id : Int32

    @[JSON::Field(key: "user_id")]
    property user_id : Int32

    @[JSON::Field(key: "title")]
    property title : String

    @[JSON::Field(key: "site_url")]
    property site_url : String

    @[JSON::Field(key: "feed_url")]
    property feed_url : String

    @[JSON::Field(key: "checked_at")]
    property checked_at : Time

    @[JSON::Field(key: "category")]
    property category : FeedCategory
  end

  class FeedEntry
    include JSON::Serializable

    @[JSON::Field(key: "id")]
    property id : Int32

    @[JSON::Field(key: "user_id")]
    property user_id : Int32

    @[JSON::Field(key: "feed_id")]
    property feed_id : Int32

    @[JSON::Field(key: "title")]
    property title : String

    @[JSON::Field(key: "url")]
    property url : String

    @[JSON::Field(key: "author")]
    property author : String

    @[JSON::Field(key: "content")]
    property content : String

    @[JSON::Field(key: "published_at")]
    property published_at : Time

    @[JSON::Field(key: "created_at")]
    property created_at : Time

    @[JSON::Field(key: "status")]
    property status : String

    @[JSON::Field(key: "starred")]
    property starred : Bool

    @[JSON::Field(key: "feed")]
    property feed : Feed
  end

  class FeedEntriesResponse
    include JSON::Serializable

    @[JSON::Field(key: "entries")]
    property entries : Array(FeedEntry)
  end
end
