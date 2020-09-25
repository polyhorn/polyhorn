#import "PLYViewController.h"
#import "PLYView.h"

@implementation PLYViewController

- (id)init {
    if ((self = [super init])) {
        _statusBarStyle = UIStatusBarStyleDefault;
    }

    return self;
}

- (void)loadView {
    self.view = [[PLYView alloc] init];
}

- (void)viewDidDisappear:(BOOL)animated {
    [super viewDidDisappear:animated];

    [self.onDidDisappear callWithArgument:nil];
}

- (void)setStatusBarStyle:(UIStatusBarStyle)statusBarStyle {
    _statusBarStyle = statusBarStyle;

    [self setNeedsStatusBarAppearanceUpdate];
}

- (UIStatusBarStyle)preferredStatusBarStyle {
    return _statusBarStyle;
}

@end