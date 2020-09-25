#import <UIKit/UIKit.h>

#import "PLYCallback.h"

@interface PLYViewController : UIViewController

@property (nonatomic, assign) UIStatusBarStyle statusBarStyle;
@property (nonatomic, strong) PLYCallback *onDidDisappear;

@end