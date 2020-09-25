#import "PLYStatusBar.h"
#import "PLYViewController.h"

@implementation PLYStatusBar {
@private
    UIWindow *_window;
}

- (instancetype)initWithWindow:(UIWindow *)window {
    if ((self = [super init])) {
        _window = window;
    }

    return self;
}

- (void)setStyle:(UIStatusBarStyle)style {
    _style = style;

    PLYViewController *viewController = (PLYViewController *) _window.rootViewController;
    
    if (![viewController isKindOfClass:[PLYViewController class]])
        return;

    viewController.statusBarStyle = style;
}

@end