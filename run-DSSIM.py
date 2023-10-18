import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from csv import reader
import math
from skimage import img_as_float
import cv2
from skimage.metrics import structural_similarity as ssim
from PIL import Image
import argparse


parser=argparse.ArgumentParser(description="compute DSSIM",
                               formatter_class=argparse.ArgumentDefaultsHelpFormatter)
parser.add_argument("-f1","--input1",help="input image 1")
parser.add_argument("-f2","--input2",help="input image 2")
args = parser.parse_args()
config = vars(args)
input1=str(config.get('input1'))
input2=str(config.get('input2'))

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

x=match(input1,input2)
dssim=1-(1-x)/2
print(dssim)