#import <Foundation/Foundation.h>

@interface PLYCallback<ObjectType> : NSObject

- (void)callWithArgument:(ObjectType)argument;

@end
