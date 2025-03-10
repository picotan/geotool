#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


static const uint8_t Weather_CLOUDY = 2;

static const uint8_t Weather_FEEBLE = 3;

static const uint8_t Weather_FOG = 7;

static const uint8_t Weather_HAIL = 5;

static const uint8_t Weather_HEAVY = 2;

static const uint8_t Weather_LITE = 1;

static const uint8_t Weather_MEDIUM = 0;

static const uint8_t Weather_NONE = 0;

static const uint8_t Weather_RAIN = 3;

static const uint8_t Weather_SLEET = 6;

static const uint8_t Weather_SNOW = 4;

static const uint8_t Weather_STORMY = 4;

static const uint8_t Weather_SUNNY = 1;

struct Area;

struct Geometry;

struct LatLon;

struct MapCoord;

template<typename T = void>
struct Option;

struct Parser;

struct PointType;

template<typename T = void, typename E = void>
struct Result;

struct String;

struct TileCoord;

struct Track;

struct TrackPoint;

struct TrackRoute;

struct TrackSegment;

template<typename T = void>
struct Vec;

struct Weather;

struct Writer;


extern "C" {

LatLon LatLon(double lat, double lon);

Option<Parser> Parser(const OsString *name);

Track Track();

TrackPoint TrackPoint(double lat, double lon);

TrackRoute TrackRoute();

void TrackSegment();

Result<Writer> Writer(const str *name);

void add(Area *self, const Area *area);

void add_point(TrackSegment *self, TrackPoint p);

void add_segment(TrackRoute *self, const TrackSegment *segment);

void append(TrackSegment *self, TrackSegment *seg);

const str *as_str(const PointType *self);

MapCoord coord(const LatLon *self);

void cut_in(TrackSegment *self, size_t i, TrackSegment *segment);

double direction(LatLon self, const LatLon *point);

double distance(const Geometry *self, const Geometry *p);

void enter(Area *self, const LatLon *location);

Option<OsString> get_file(const LatLon *geo, uint32_t zoom);

bool has_extension(const TrackPoint *self);

void insert_at(TrackSegment *self, TrackPoint p, size_t i);

Area invalid();

bool is_in(const Area *self, const Area *area);

LatLon latlon(MapCoord coord);

LatLon latlon_from_tile(const TileCoord *self);

Option<Track> open(Parser *self);

double per_mill(const Geometry *self, const Geometry *p);

Option<Vec<TrackRoute>> process_gpx(Parser *self);

void remove_at(TrackSegment *self, TrackPoint p, size_t i);

void set_comment(TrackPoint *self, String comment);

void set_name(TrackPoint *self, String name);

bool split_segment_at(TrackRoute *self, const TrackPoint *point);

TileCoord tile_from_latlon(const LatLon *l, uint32_t z);

Vec<OsString> type_str(const TrackPoint *self);

Weather weather(uint8_t str, uint8_t state);

OsString weather_str(const TrackPoint *self);

void write(Writer *self, const Track *track);

}  // extern "C"
