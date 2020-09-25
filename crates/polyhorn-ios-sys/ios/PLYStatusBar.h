#import <UIKit/UIKit.h>

@interface PLYStatusBar : NSObject

- (instancetype)initWithWindow:(UIWindow *)window;

@property (nonatomic, assign) UIStatusBarStyle style;

@end