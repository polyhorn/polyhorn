#import <UIKit/UIKit.h>

#import "PLYGeometry.h"
#import "PLYLayout.h"

@interface PLYScrollView : UIScrollView

@property (nonatomic, strong, nullable) PLYLayout *layout;
@property (nonatomic, strong, nullable) PLYLayout *contentLayout;
@property (nonatomic, assign) PLYCornerRadii cornerRadii;
@property (nonatomic, assign) PLYDimensionByEdge scrollPadding;
@property (nonatomic, assign) PLYDimensionByEdge scrollbarPadding;
@property (nonatomic, strong, nullable) UIColor *opaqueBackgroundColor;

@end
