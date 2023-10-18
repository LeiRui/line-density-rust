import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from csv import reader
import math
from scipy import signal
from tsdownsample import MinMaxLTTBDownsampler,M4Downsampler,EveryNthDownsampler,LTTBDownsampler,MinMaxDownsampler
import cv2
from skimage.metrics import structural_similarity as ssim
from skimage import img_as_float
import cv2
from skimage.metrics import structural_similarity as ssim
from PIL import Image

def match(imfil1,imfil2):    
    img1=cv2.imread(imfil1)    
    (h,w)=img1.shape[:2]    
    img2=cv2.imread(imfil2)    
    resized=cv2.resize(img2,(w,h))    
    (h1,w1)=resized.shape[:2]    
    # print(img1.dtype)
    img1=img_as_float(cv2.cvtColor(img1, cv2.COLOR_BGR2GRAY)) # img_as_float: the dtype is uint8, means convert [0, 255] to [0, 1]
    img2=img_as_float(cv2.cvtColor(resized, cv2.COLOR_BGR2GRAY))
    return ssim(img1,img2,data_range=img2.max() - img2.min())

x=match('output-i1-k100-w400-h400-ufalse-dfalse.png','output-i1-k100-w400-h400-ufalse-dtrue.png')
dssim=1-(1-x)/2
print(dssim)