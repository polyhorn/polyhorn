#import <Foundation/Foundation.h>
#import <QuartzCore/QuartzCore.h>

#import "PLYCallback.h"

@interface PLYAnimationHandle : NSObject

- (_Nonnull instancetype)initWithLayer:(CALayer * _Nonnull)layer
                                   key:(NSString * _Nonnull)key;

@property (nonatomic, strong, nonnull, readonly) CALayer *layer;
@property (nonatomic, strong, nonnull, readonly) NSString *key;
@property (nonatomic, strong, nullable) PLYCallback *onStop;

@end