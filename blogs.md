I wrote this engine taking inspiration from Euclideon unlimited detail engine and Atomontage engine, which both are closed source.

Euclideon claims that they can render an unlimited number of point cloud data using just a laptop at 20 fps. That is not using a GPU, if whatever they are saying is true, then they are using some shortcut algorithms which don't use a lot of mathematical calculations. Though euclideon explicitly mentioned that they are not using voxel, neither raytracing.


Atomontage engine clearly stated that they are using voxels and can render the 1000 fps using an old laptop. 
(http://www.atomontage.com/sshots/ae_kladno_600fps.jpg)

It seems atomontage engine is using the GPU to achieve this. 

If euclideon uses the GPU, then they may achive the same effect.


## How to achieve such performance

A contrary to common beliefs, achieving performance is achieved through the use of sophiscticated clever algorithmns, but I believe that the way to achieve this is to employ the least possible calculations needed to achieve a goal.

Euclideon mentions that they are not doing multiplication, so probably they are only using bitset operations




