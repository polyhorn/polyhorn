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
    PLYPoint topLeft;
    PLYPoint topRight;
    PLYPoint bottomRight;
    PLYPoint bottomLeft;
} PLYCornerRadii;

typedef struct PLYDimensionLayoutAxisX {
    bool independent;
    PLYDimension start;
    PLYDimension end;
} PLYDimensionLayoutAxisX;

typedef struct PLYDimensionLayoutAxisY {
    PLYDimension top;
    PLYDimension bottom;
} PLYDimensionLayoutAxisY;

typedef struct PLYDimensionByEdge {
    PLYDimensionLayoutAxisX horizontal;
    PLYDimensionLayoutAxisY vertical;
} PLYDimensionByEdge;
