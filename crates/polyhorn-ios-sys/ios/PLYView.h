#import <UIKit/UIKit.h>

#import "PLYAnimationHandle.h"
#import "PLYCallback.h"
#import "PLYGeometry.h"
#import "PLYKeyframeAnimation.h"
#import "PLYLayout.h"
#import "PLYLayoutEvent.h"

@interface PLYView : UIView

- (PLYAnimationHandle * _Nonnull)addKeyframeAnimation:(PLYKeyframeAnimation * _Nonnull)animation
                                           forKeyPath:(NSString * _Nonnull)keyPath;

/* Layout */
@property (nonatomic, strong, nullable) PLYLayout *layout;

/* Style */
@property (nonatomic, assign) PLYCornerRadii cornerRadii;
@property (nonatomic, strong, nullable) UIColor *opaqueBackgroundColor;

/* Event Listeners */
@property (nonatomic, strong, nullable) PLYCallback *onPointerCancel;
@property (nonatomic, strong, nullable) PLYCallback *onPointerDown;
@property (nonatomic, strong, nullable) PLYCallback *onPointerUp;
@property (nonatomic, strong, nullable) PLYCallback<PLYLayoutEvent *> *onLayout;

@end
