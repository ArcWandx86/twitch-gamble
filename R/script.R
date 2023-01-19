library(ggplot2)
setwd("~/projects/twitch-gamble/R")
df <- read.csv("runs.csv")
mean(df$End.Balance)
# table(df$Dead.After)
# table(df$Turns.to.Target)
# head(table(df$End.Balance))
ggplot(df, aes(x=Turns.to.Target)) + geom_histogram(binwidth = 7, color = "#cccc33", fill = "#FF6666") + geom_vline(aes(xintercept=mean(Turns.to.Target)), color="grey", linetype="dashed", linewidth=0.5)
ggsave("turns_to_target.jpg")

ggplot(df, aes(x=End.Balance)) +geom_histogram(binwidth = 100000000, color = "#cccc33", fill = "#FF6666") + geom_vline(aes(xintercept=mean(End.Balance)), color="grey", linetype="dashed", linewidth=0.5) + scale_y_continuous(trans='log10')
ggsave("end_balance.jpg")

ggplot(df, aes(x=End.Balance)) +geom_histogram(binwidth = 1, color = "#cccc33", fill = "#6666FF") + geom_vline(aes(xintercept=mean(End.Balance)), color="grey", linetype="dashed", linewidth=0.5) + scale_y_continuous(trans='log10') +scale_x_continuous(trans='log10')
ggsave("end_balance_log_x.jpg")

