#import <CoreGraphics/CoreGraphics.h>

typedef enum {
    PLYDimensionKindPixels,
    PLYDimensionKindPercentage,
} PLYDimensionKind;

typedef struct {
    PLYDimensionKind kind;
    CGFloat value;
} PLYDimension;

typedef struct {
    PLYDimension x;
    PLYDimension y;
} PLYPoint;

typedef struct {
    PLYPoint topLeading;
    PLYPoint topTrailing;
    PLYPoint bottomTrailing;
    PLYPoint bottomLeading;
} PLYCornerRadii;