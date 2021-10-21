#include <bits/stdc++.h>
#include <iostream>
#include <iostream>


using namespace std;

const int MAX_N = 1e5 + 1;
const int MOD = 1e9 + 7;
const int INF = 1e9;
const long long LINF = 1e18;


//Global variables
const int N = 26;
int D = 365;
vector<int> c (N);
vector<vector<int>> s(D);


//Greedy solution
vector<int> greedy()
{
	vector<int> days_no_contest (N, 1);
	vector<int> ans (D, 1);
	
	for( int i = 0; i < D; i++)
	{
		long long M = -INF;
		
		long long total_penalty = 0;
		for( int k = 0; k < N; k++){ total_penalty += (c[k] * days_no_contest[k]); }
		
		for( int k = 0; k < N; k++)
		{
			long long score = 1e6 + s[i][k] - total_penalty + (c[k] * days_no_contest[k]);
			if( score > M)
			{
				M = score;
				ans[i] = k + 1;
			}
		}
		
		for( int k = 0; k < N; k++)
		{
			days_no_contest[k] += 1;
		}
		
		days_no_contest[ans[i] - 1] = 1;
	}
	
	return ans;	
}

//Better greedy solution
//Computes the score of the answer and computes de penalty for not playing the contest for k more days
long long evaluate(vector<int> ans, int k)
{
	vector<int> last (N, 0);
	int len = ans.size();
	
	long long score = 0;
	for( int d = 0; d < len; d++)
	{
		last[ans[d] - 1] = d + 1;
		score = 1e6 + s[d][ans[d] - 1];
		for( int i = 0; i < N; i++)
		{
			score -= (c[i] * (d + 1 - last[i]));
		}
	}

	for( int d = len; d < len + k && d < D; d++)
	{
		for( int i = 0; i < N; i++)
		{
			score -= (c[i] * (d + 1 - last[i]));
		}
	}
	
	return score;
}

vector<int> better_greedy()
{
	vector<int> ans;
	int k = 5;
	
	for( int i = 0; i < D; i++)
	{
		long long M = -INF;
		int best = 1;
		for( int j = 0; j < N; j++)
		{
			ans.push_back(j + 1);
			long long s = evaluate(ans, k);
			if(M < s)
			{
				M = s;
				best = j + 1;
			}
			ans.pop_back();
		}
		
		ans.push_back(best);
	}
	
	return ans;	
}


//Genetic algorithm starts here -------------------------------------------------------

// function generator:
int RandomNumber () { return (1 + rand()%25); }

//Function to make up to k mutations on the new childs
vector<int> mutation(vector<int> chrom, int k)
{
	//srand(time(NULL));
	
	vector<int> new_chrom = chrom;
	
	for( int i = 0; i < k; i++)
	{
		chrom[rand() % D] = rand() % N;
	}
	
	return new_chrom;
}

//Create a random chromosome
vector<int> create_chromosome(int size)
{
	/*
	vector<int> chrom (size, 0);
	generate(chrom.begin(), chrom.end(), RandomNumber);
	*/
	
	//Make a new chromosome mutating a greedy solution
	vector<int> chrom = better_greedy();
	chrom = mutation(chrom, 50);
	
	return chrom;
}

//Computes the score
long long get_score(vector<int> chrom)
{
	vector<int> days_no_contest (N, 1);
	
	long long score = 0;
	for( int i = 0; i < D; i++)
	{
		
		long long day_score = 1e6 + s[i][chrom[i] - 1];
		for( int k = 0; k < N; k++)
		{
			day_score -= (c[k] * days_no_contest[k]);
		}
		
		if(day_score > 0) { score += day_score; }
		
		for( int k = 0; k < N; k++)
		{
			days_no_contest[k] += 1;
		}
		
		days_no_contest[chrom[i] - 1] = 1;
	}
	
	return score;
}

//Returns the score normalized from 0 to 1
double score(vector<int> chrom)
{
	return (double) (get_score(chrom) / 372300000.0);
}

//Compare function
bool comp(vector<int> a, vector<int> b) { return (score(a) > score(b)); }

//Function to get the parents for the next generation
vector<vector<int>> selection(vector<vector<int>> chromosome_list)
{
	float GRADED_RETAIN_PERCENT = 0.3;
	
	int parents_len = (int)(GRADED_RETAIN_PERCENT * chromosome_list.size());
	vector<vector<int>> parents;

	sort(chromosome_list.begin(), chromosome_list.end(), comp);
	
	for( int i = 0; i < parents_len; i++)
	{
		parents.push_back(chromosome_list[i]);
	}
	
	return parents;
}

//Function to make to parents generate a child
vector<int> crossover(vector<int> parent1, vector<int> parent2)
{
	vector<int> child (D, 1);
	for( int i = 0; i < D; i++)
	{
		if(rand() % 2)
		{
			child[i] = parent1[i];
		}
		else
		{
			child[i] = parent2[i];
		}
	}
	
	return child;
}

//Function that creates a new generation
vector<vector<int>> generation(vector<vector<int>> population)
{
	//srand(time(NULL));
	
	vector<vector<int>> select = selection(population);
	
	//Reproduction
	vector<vector<int>> children (0);
	while( children.size() < population.size() - select.size())
	{
		//Crossover
		int a = rand() % select.size();
		int b = rand() % select.size();
		while( b == a ) { b = rand() % select.size(); }
		vector<int> child = crossover(select[a], select[b]);
		
		//Mutate
		child = mutation(child, rand() % 30);
		children.push_back(child);
	}

	select.insert(select.end(), children.begin(), children.end());
	
	return select;
}

//Genetic algo
vector<int> schedule()
{
	int chrom_size = D;
	int population_size = 25;
	int max_generation = 500;
	
	srand(time(NULL));
	
	//Create the base population
	vector<vector<int>> population (population_size);
	population[0] = better_greedy();
	for( int i = 1; i < population_size; i++) { population[i] = create_chromosome(chrom_size); }
	
	for( int i = 0; i < max_generation; i++)
	{
		population = generation(population);
	}
	
	sort(population.begin(), population.end(), comp);
	
	return population[0];
}

//Genetic algorithm ends here ----------------------------------------------------------


int main() {
    cin >> D;
    
    for (auto& e : c) { cin >> e; }
    
	for (auto& row : s)
	{
		row.resize(N);
		for (auto& e : row) { cin >> e; }
	}
	
	vector<int> sol = better_greedy();
	for(auto& e : sol) { cout << e << endl; }

    return 0;
}
