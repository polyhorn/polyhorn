#import <UIKit/UIKit.h>

#import "PLYCallback.h"
#import "PLYLayout.h"

@interface PLYKeyboardAvoidingView : UIView

@property (nonatomic, strong, nullable) PLYLayout *layout;
@property (nonatomic, strong, nullable) PLYCallback *onKeyboard;

@end