# Rust-CS128-HonorsProject
## Group Name - EnergiAI

## Group Members:
 
1. Aryan Gupta - aryan10
2. Ritvik Sood - ritviks3
3. Aaditya Voruganti - av34

## Project Introduction:
  
The EnergiAI project aims to use AI to predict consumer energy demand and adjust energy supply accordingly to improve efficiency. To achieve this, we propose using   a time series classification (TSC) model in Rust to analyze historical energy usage data, weather patterns, and other relevant factors to predict future energy       demand.

Specifically, we can use the K-Nearest Neighbors (KNN) algorithm, which is well-suited for analyzing time series data, to predict the energy demand class of new  data points based on the k closest matches to time-based data points. Rust libraries such as the Time Series Library (crate) and Rusty Machine (crate) can be utilized to facilitate data preprocessing, feature extraction, and model training.

The main goal of this project is to develop an energy consumption model that can adjust energy supply to meet predicted demand, reducing waste and increasing efficiency. The TSC model can help to improve the accuracy of energy demand predictions, enabling better management of energy supply. We believe that Rust is well-suited for this project due to its performance and reliability, which are critical factors for an energy management system. Additionally, we plan to make use of some of its useful features like network communication and multi-threading. By improving energy efficiency, the EnergiAI project can have a positive impact on the environment and help companies and consumers save money on energy costs.

## Technical Overview:
  
 1. Researching the way energy is used by society currently and finding public datasets to use for our ML model
 2. Using a Time Series classfication model to understand trends in energy consumption
 3. Creating a UI to understand these trends more easily
 4. Judging the main parameters that effect the amount energy used in a paticular geographical location or by a type of household/Business
 5. Optimizing the energy flow by using a Djixtra based algorithm.
 6. Finally, develop an energy consumption model that can adjust energy supply to meet predicted demand, reducing waste and increasing efficiency

## Possible Challenges:

1. Gaining experience working with and implementing ML models in Rust beyond the basic KNN
2. Understanding and correctly making use of the time series and KNN crates in Rust
3. Learning to work with Rusty Machine, a general purpose machine learning library for Rust
4. Parsing data and working with large datasets in Rust
5. Analyzing predictions to draw meaningful conclusions about energy production and consumption

## References:

We will draw some inspiration from the homework about the K-Nearest Neighbors (KNN) algorithm in order to implement our own ML algorithms for our project. Additionally, we will be using the following crates and libraries:

1. timeseries: https://docs.rs/timeseries/latest/timeseries/
2. knn: https://docs.rs/knn/latest/knn/ 
3. Rusty Machine: https://athemathmo.github.io/rusty-machine/doc/rusty_machine/index.html

## Results


Mean Squared Error: 0.05213198362988421
![alt text](images/my_image.png)


