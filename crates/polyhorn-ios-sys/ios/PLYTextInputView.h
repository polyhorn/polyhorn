#import <UIKit/UIKit.h>

#import "PLYCallback.h"
#import "PLYLayout.h"

@interface PLYTextInputView : UIView <UITextFieldDelegate>

@property (nonatomic, strong) PLYLayout *layout;
@property (nonatomic, strong) PLYCallback *onChange;
@property (nonatomic, strong) NSAttributedString *attributedPlaceholder;
@property (nonatomic, strong) NSString *text;

@end