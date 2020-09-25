#import <Foundation/Foundation.h>
#import <QuartzCore/QuartzCore.h>

#import "PLYAnimationHandle.h"

@interface PLYAnimationDelegate : NSObject <CAAnimationDelegate>

- (instancetype)initWithHandle:(PLYAnimationHandle *)handle;

@property (nonatomic, weak, readonly) PLYAnimationHandle *handle;

@end
