#!/bin/bash

export API_KEY=$1

function call() {
    if [ -z "$3" ]; then
        curl "https://api.themoviedb.org/3$1?api_key=$API_KEY" | jq . > assets/"$2".json
    else
        curl "https://api.themoviedb.org/3$1?api_key=$API_KEY&$3" | jq . > assets/"$2".json
    fi
}

# certification
call /certification/movie/list certification-movie-list
call /certification/tv/list certification-tv-list

# collection
call /collection/10 collection-details

# changes
call /movie/changes movie-all-changes
call /tv/changes tv-all-changes
call /person/changes people-all-changes

# companies
call /company/1 company-details
call /company/1/alternative_names company-alternative-names
call /company/1/images company-images

# configuration
call /configuration/countries configuration-countries
call /configuration/jobs configuration-jobs
call /configuration/languages configuration-languages

# credits
call /credit/52542282760ee313280017f9 credit-details

# genres
call /genre/movie/list genre-movie-list
call /genre/tv/list genre-tv-list

# movies
call /movie/550 movie-details
call /movie/550/alternative_titles movie-alternative-titles
call /movie/550/changes movie-single-changes start_date=2022-10-10
call /movie/550/credits movie-credits
call /movie/550/external_ids movie-external-ids
call /movie/550/images movie-images
call /movie/550/keywords movie-keywords
call /movie/550/lists movie-lists
call /movie/550/release_dates movie-release-dates
call /movie/550/recommendations movie-recommendations
call /movie/550/reviews movie-reviews
call /movie/550/similar movie-similar
call /movie/550/translations movie-translations
call /movie/550/videos movie-videos
call /movie/550/watch/providers movie-watch-providers
call /movie/latest movie-latest
call /movie/now_playing movie-now-playing
call /movie/popular movie-popular
call /movie/top_rated movie-top-rated
call /movie/upcoming movie-upcoming

# person
call /person/287 person-details

# search
call /search/movie search-movie query=RRRrrrr
call /search/tv search-tv query=game+of+thrones

# TV shows
call /tv/1399 tv-details
call /tv/2 tv-details-complex
call /tv/1399/similar tv-similar
call /tv/1399/keywords tv-keywords
call /tv/1399/content_ratings tv-content-ratings
call /tv/1399/external_ids tv-external-ids
call /tv/1399/aggregate_credits tv-aggregate-credits
call /tv/1399/season/1 tv-season-details
call /tv/1399/season/1/episode/1 tv-episode-details

# watch providers
call /watch/providers/movie watch-provider-movie-list
call /watch/providers/tv watch-provider-tv-list
